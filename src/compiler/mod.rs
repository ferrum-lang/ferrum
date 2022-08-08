mod ast;
mod generator;
mod lexer;
mod semantics;

use crate::io;

use std::path;

use anyhow::Result;

pub fn compile(filepath: &path::PathBuf) -> Result<String> {
    let mut syntax_tree = compile_to_unchecked_ast(filepath)?;

    semantics::fix_and_validate(&mut syntax_tree)?;

    return generator::generate_rust(syntax_tree);
}

fn compile_to_unchecked_ast(filepath: &path::PathBuf) -> Result<ast::AST> {
    let tokens = lexer::tokenize(filepath)?;

    println!("Tokens: \n\n{tokens}\n\n");

    let syntax_tree = ast::parse_ast(tokens)?;

    // TODO: Recursive compilation to resolve imports

    return Ok(syntax_tree);
}

