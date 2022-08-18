mod generator;
mod lexer;
mod parser;
mod semantics;

use std::path;

use anyhow::Result;

pub fn compile(filepath: &path::PathBuf) -> Result<String> {
    let ast = compile_to_unchecked_ast(filepath)?;

    let rs_ast = semantics::translate(ast)?;

    return generator::generate_rust(rs_ast);
}

fn compile_to_unchecked_ast(filepath: &path::PathBuf) -> Result<parser::AST> {
    let tokens = lexer::tokenize(filepath)?;

    // println!("Tokens: \n\n{tokens}\n\n");

    let ast = parser::parse_ast(tokens)?;

    // TODO: Recursive compilation to resolve imports

    return Ok(ast);
}

