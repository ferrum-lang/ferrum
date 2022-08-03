mod ast;
mod generator;
mod lexical;
mod semantics;

use anyhow::Result;

pub fn compile(input_contents: String) -> Result<String> {
    let tokens = lexical::parse_tokens(input_contents)?;

    let syntax_tree = ast::parse_ast(tokens)?;

    semantics::validate(&syntax_tree)?;

    return generator::generate_rust(syntax_tree);
}

