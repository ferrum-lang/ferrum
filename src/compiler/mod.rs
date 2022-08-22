mod generator;
mod lexer;
mod parser;
mod semantics;

use std::path;

use anyhow::Result;

pub fn compile(filepath: &path::PathBuf) -> Result<String> {
    let input_ast = compile_to_input_ast(filepath)?;

    let semantic_ast = semantics::translate(input_ast)?;

    let rust = generator::generate_rust(semantic_ast);

    // todo!("\n\nRust:\n{rust}\n");

    return Ok(rust);
}

fn compile_to_input_ast(filepath: &path::PathBuf) -> Result<parser::AST> {
    let tokens = lexer::tokenize(filepath)?;

    // println!("Tokens: \n\n{tokens}\n\n");

    let ast = parser::parse_ast(tokens)?;

    // TODO: Recursive compilation to resolve imports

    return Ok(ast);
}

