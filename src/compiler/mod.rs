mod generator;
mod lexer;
mod parser;
mod symbols;
mod syntax;
mod tokens;

use super::error::Error;

pub fn compile(contents: String) -> Result<String, Error> {
    let tokens = lexer::lex_into_tokens(contents)?;

    let symbols = symbols::symolize_tokens(tokens)?;

    let syntax_tree = parser::parse_symbols(symbols)?;

    return generator::generate_rust(syntax_tree);
}
