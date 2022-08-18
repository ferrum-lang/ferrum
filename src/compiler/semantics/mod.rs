use super::parser as p;

use anyhow::Result;

pub struct SemanticAST {}

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

    return Ok(SemanticAST {});
}

