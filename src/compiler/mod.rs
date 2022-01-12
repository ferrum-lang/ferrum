mod ast;
mod interpreter;
mod lexer;
mod parser;
mod tokens;

use super::error::Error;

pub fn compile_to_rust(contents: String) -> Result<String, Error> {
    let unparsed_tokens = lexer::lex_into_tokens(contents)?;

    let tokens = parser::parse_tokens(unparsed_tokens)?;

    let ast = ast::build_from_tokens(tokens)?;

    return interpreter::generate_rust(ast);
}
