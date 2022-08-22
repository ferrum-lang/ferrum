use super::*;
use super::super::parser as p;

use anyhow::Result;

pub fn validate_and_contextualize(ast: p::AST) -> Result<SemanticAST> {
    let mut has_explicit_main = None;

    for node in ast.nodes.into_iter() {
        match node {
            p::RootNode::Statement(_) => {
                if let Some(has_main) = has_explicit_main {
                    if has_main {
                        panic!("either top-level statements OR main function");
                    }
                } else {
                    has_explicit_main = Some(false);
                }
            },
            p::RootNode::Definition(p::Definition::Function(def_fn)) if def_fn.signature.name.as_str() == "main" => {
                if let Some(has_main) = has_explicit_main {
                    if has_main {
                        panic!("only 1 main function allowed");
                    } else {
                        panic!("either top-level statements OR main function");
                    }
                } else {
                    has_explicit_main = Some(true);
                }
            },
            _ => {},
        }
    }

    if has_explicit_main.is_none() {
        has_explicit_main = Some(false);
    }

    // println!("has_explicit_main: {has_explicit_main:?}");

    return Ok(SemanticAST {
        mods: vec![
            Mod {
                is_public: false,
                name: "fe_prelude".to_string(),
            }
        ],
        uses: vec![
            Use {
                is_public: false,
                tree: UseTree::Path(UsePath {
                    ident: "fe_prelude".to_string(),
                    tree: Box::new(UseTree::Glob),
                })
            },
        ],
        static_consts: vec![],
        lazy_static_consts: vec![],
        items: vec![
            Item::Fn(ItemFn {
                is_public: true,
                signature: FnSignature {
                    is_const: false,
                    is_async: false,
                    name: "main".to_string(),
                    generics: Generics {
                        params: vec![],
                    },
                    params: vec![],
                    return_type: ReturnType::Default,
                },
                block: Box::new(Block {
                    statements: vec![
                        Statement::Semi(Expr::Call(ExprCall {
                            func: Box::new(Expr::Path(ExprPath {
                                path: Path {
                                    segments: vec![
                                        PathSegment {
                                            ident: "print".to_string(),
                                            arguments: PathArguments::None,
                                        }
                                    ],
                                }
                            })),
                            args: vec![
                                Expr::Lit(ExprLit {
                                    literal: Literal::Str("hello world".to_string()),
                                }),
                                Expr::Lit(ExprLit {
                                    literal: Literal::Bool(true),
                                }),
                            ],
                        }))
                    ],
                }),
            })
        ],
    });
}

