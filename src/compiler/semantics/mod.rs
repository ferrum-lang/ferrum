use super::parser::{AST, *};

use anyhow::Result;

pub fn translate(ast: AST) -> Result<syn::File> {
    let mut has_explicit_main = None;

    for node in ast.nodes.into_iter() {
        match node {
            RootNode::Statement(_) => {
                if let Some(has_main) = has_explicit_main {
                    if has_main {
                        panic!("either top-level statements OR main function");
                    }
                } else {
                    has_explicit_main = Some(false);
                }
            },
            RootNode::Definition(Definition::Function(def_fn)) if def_fn.signature.name.as_str() == "main" => {
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

    println!("has_explicit_main: {has_explicit_main:?}");

    let file = syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![
            // syn::Item::Fn(syn::ItemFn {
            //     attrs: vec![],
            //     vis: syn::Visibility::Public(syn::VisPublic {
            //         pub_token: syn::Token![pub],
            //     }),
            //     block: 
            // }),
        ],
    };

    return Ok(file);
}

