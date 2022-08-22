mod def_fn;
mod import;
mod static_const;

pub use def_fn::*;
pub use import::*;
pub use static_const::*;

use super::*;
use super::super::parser as p;

use anyhow::Result;

pub fn translate(mut p_ast: p::AST) -> Result<SemanticAST> {
    move_top_level_into_main(&mut p_ast)?;

    // println!("\n\n{p_ast:?}\n\n");

    let mut ast = SemanticAST {
        mods: vec![],
        uses: vec![],
        static_consts: vec![],
        lazy_static_consts: vec![],
        items: vec![],
    };

    ast.mods.push(Mod {
        is_public: false,
        name: "fe_prelude".to_string(),
    });

    ast.uses.push(
        Use {
            is_public: false,
            tree: UseTree::Path(UsePath {
                ident: "fe_prelude".to_string(),
                tree: Box::new(UseTree::Glob),
            })
        });

    for import in p_ast.imports.into_iter() {
        translate_import(&mut ast, import)?;
    }

    for static_const in p_ast.static_consts.into_iter() {
        translate_static_const(&mut ast, static_const)?;
    }

    for node in p_ast.nodes.into_iter() {
        match node {
            p::RootNode::Definition(definition) => match definition {
                p::Definition::Type(def_type) => todo!(),
                p::Definition::Function(def_fn) => translate_def_fn(&mut ast, def_fn)?,
            },
            p::RootNode::Statement(_) => unreachable!("All root statements should be moved into main"),
        }
    }

    return Ok(ast);

    // return Ok(SemanticAST {
    //     mods: vec![
    //         Mod {
    //             is_public: false,
    //             name: "fe_prelude".to_string(),
    //         }
    //     ],
    //     uses: vec![
    //         Use {
    //             is_public: false,
    //             tree: UseTree::Path(UsePath {
    //                 ident: "fe_prelude".to_string(),
    //                 tree: Box::new(UseTree::Glob),
    //             })
    //         },
    //     ],
    //     static_consts: vec![],
    //     lazy_static_consts: vec![],
    //     items: vec![
    //         Item::Fn(ItemFn {
    //             is_public: true,
    //             signature: FnSignature {
    //                 is_const: false,
    //                 is_async: false,
    //                 name: "main".to_string(),
    //                 generics: Generics {
    //                     params: vec![],
    //                 },
    //                 params: vec![],
    //                 return_type: ReturnType::Default,
    //             },
    //             block: Box::new(Block {
    //                 statements: vec![
    //                     Statement::Semi(Expr::Call(ExprCall {
    //                         func: Box::new(Expr::Path(ExprPath {
    //                             path: Path {
    //                                 segments: vec![
    //                                     PathSegment {
    //                                         ident: "print".to_string(),
    //                                         arguments: PathArguments::None,
    //                                     }
    //                                 ],
    //                             }
    //                         })),
    //                         args: vec![
    //                             Expr::Lit(ExprLit {
    //                                 literal: Literal::Str("hello world".to_string()),
    //                             }),
    //                             Expr::Lit(ExprLit {
    //                                 literal: Literal::Bool(true),
    //                             }),
    //                         ],
    //                     }))
    //                 ],
    //             }),
    //         })
    //     ],
    // });
}

fn move_top_level_into_main(ast: &mut p::AST) -> Result<()> {
    let mut main_fn: Option<(bool, p::DefFnSignature, p::Block)> = None;

    let mut nodes = vec![];

    for node in ast.nodes.clone().into_iter() {
        match node {
            p::RootNode::Statement(stmt) => match &mut main_fn {
                Some((has_main, _def_sig, block)) => {
                    if *has_main {
                        panic!("either top-level statements OR main function");
                    }

                    block.statements.push(stmt);
                },
                None => {
                    main_fn = Some(
                        (
                            false,
                            p::DefFnSignature {
                                is_async: p::MaybeBool::False,
                                is_public: false,
                                generics: None,
                                name: "main".to_string(),
                                params: vec![],
                                return_type: None,
                            },
                            p::Block {
                                statements: vec![stmt],
                            },
                        )
                    );
                },
            },
            p::RootNode::Definition(p::Definition::Function(def_fn)) if def_fn.signature.name.as_str() == "main" => {
                if let Some((has_main, _, _)) = main_fn {
                    if has_main {
                        panic!("only 1 main function allowed");
                    } else {
                        panic!("either top-level statements OR main function");
                    }
                }

                let block = match def_fn.r#impl {
                    p::DefFnImpl::Block(block) => block,
                    p::DefFnImpl::Expression(expr) => p::Block {
                        statements: vec![p::Statement::Expression(expr)],
                    },
                };

                main_fn = Some((true, def_fn.signature, block));
            },
            _ => nodes.push(node),
        }
    }

    let (_, signature, block) = main_fn.unwrap_or_else(
        || (
            false,
            p::DefFnSignature {
                is_async: p::MaybeBool::False,
                is_public: false,
                generics: None,
                name: "main".to_string(),
                params: vec![],
                return_type: None,
            },
            p::Block {
                statements: vec![],
            }
        )
    );

    nodes.insert(0, p::RootNode::Definition(p::Definition::Function(p::DefFn {
        signature,
        r#impl: p::DefFnImpl::Block(block),
    })));

    ast.nodes = nodes;

    return Ok(());
}

