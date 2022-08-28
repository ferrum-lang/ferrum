mod def_fn;
mod import;
mod static_const;
mod verify_locals;

pub use def_fn::*;
pub use import::*;
pub use static_const::*;
pub use verify_locals::*;

use super::*;
use super::super::parser as p;

use std::collections::HashMap;

use anyhow::Result;

pub type FnParamsMap = HashMap<String, Vec<(String, Option<Box<p::Expression>>)>>;

pub fn translate(mut p_ast: p::AST) -> Result<SemanticAST> {
    move_top_level_into_main(&mut p_ast)?;

    verify_locals(&p_ast)?;

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

    let fn_params_map: FnParamsMap = build_fn_args_map(&p_ast.nodes);

    for node in p_ast.nodes.into_iter() {
        match node {
            p::RootNode::Definition(definition) => match definition {
                p::Definition::Type(def_type) => todo!(),
                p::Definition::Function(def_fn) => translate_def_fn(&fn_params_map, &mut ast, def_fn)?,
            },
            p::RootNode::Statement(_) => unreachable!("All root statements should be moved into main"),
        }
    }

    return Ok(ast);
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

fn build_fn_args_map(nodes: &Vec<p::RootNode>) -> FnParamsMap {
    let mut map: FnParamsMap = HashMap::new();

    for node in nodes.iter() {
        match node {
            p::RootNode::Definition(p::Definition::Function(p::DefFn { signature, .. })) => {
                let (fn_name, params) = (
                    signature.name.clone(),
                    signature.params.clone()
                        .into_iter()
                        .map(|param| (param.name, param.default))
                        .collect()
                );

                map.insert(fn_name, params);
            },
            p::RootNode::Definition(p::Definition::Type(p::DefType::Class(p::DefClass { methods, impls, .. }))) => {
                for method in methods.iter() {
                    let (fn_name, params) = (
                        method.signature.name.clone(),
                        method.signature.params.clone()
                            .into_iter()
                            .map(|param| (param.name, param.default))
                            .collect()
                    );

                    map.insert(fn_name, params);
                }

                for r#impl in impls.iter() {
                    for method in r#impl.methods.iter() {
                        let (fn_name, params) = (
                            method.signature.name.clone(),
                            method.signature.params.clone()
                            .into_iter()
                            .map(|param| (param.name, param.default))
                            .collect()
                            );

                        map.insert(fn_name, params);
                    }
                }
            },
            p::RootNode::Definition(p::Definition::Type(p::DefType::Interface(p::DefInterface { methods, .. }))) => {
                for method in methods.iter() {
                    let (fn_name, params) = (
                        method.name.clone(),
                        method.params.clone()
                            .into_iter()
                            .map(|param| (param.name, param.default))
                            .collect()
                    );

                    map.insert(fn_name, params);
                }
            },
            _ => {},
        };
    }

    return map;
}

