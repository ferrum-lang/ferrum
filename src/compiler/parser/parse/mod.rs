mod definitions;
mod error;
mod imports;
mod statements;
mod static_consts;

pub use definitions::*;
pub use error::*;
pub use imports::*;
pub use statements::*;
pub use static_consts::*;

use crate::utils::stack::Stack;

use super::ast::*;
use super::super::lexer::*;

use anyhow::Result;

pub fn parse_ast(tokens: Tokens) -> Result<AST> {
    let mut tokens = Stack::from_top_to_bottom_vec(tokens.value);

    let mut ast = AST {
        imports: vec![],
        static_consts: vec![],
        nodes: vec![],
    };

    println!("\n{tokens:?}\n");

    parse_imports(&mut ast, &mut tokens)?;
    
    println!("\n{tokens:?}\n");
    
    parse_static_consts(&mut ast, &mut tokens)?;
    
    println!("\n{tokens:?}\n");
    
    parse_nodes(&mut ast, &mut tokens)?;
    
    println!("\n{tokens:?}\n");

    todo!("\n{ast:?}\n");
    // return Ok(ast);
}

pub fn ignore_new_lines(tokens: &mut Stack<TokenData>) -> Option<TokenData> {
    let mut new_line = None;

    while let Some(token) = tokens.peek() {
        match token.value {
            Token::NewLine => {
                new_line = tokens.pop();
            }
            _ => break,
        }
    }

    return new_line;
}

fn parse_nodes(
    ast: &mut AST,
    tokens: &mut Stack<TokenData>,
) -> Result<()> {
    while let Some(token) = tokens.peek() {
        if token.value == Token::NewLine {
            tokens.pop();
            continue;
        }

        let is_definition = match token.value {
            Token::Keyword(Keyword::Pub) => true,
            Token::Keyword(Keyword::Fn) => true,
            Token::Keyword(Keyword::Struct) => true,
            Token::Keyword(Keyword::Class) => true,
            Token::Keyword(Keyword::Interface) => true,
            Token::Keyword(Keyword::Enum) => true,
            Token::Keyword(Keyword::Errors) => true,
            _ => false,
        };

        if is_definition {
            parse_definition(ast, tokens)?;
        } else {
            parse_statement(ast, tokens)?;
        }
    }

    return Ok(());
}

