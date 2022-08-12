use super::*;

use super::super::ast::{self, *};

use super::super::super::lexer::{self, *};

use anyhow::Result;

pub fn parse_static_consts(
    ast: &mut AST,
    tokens: &mut Stack<TokenData>,
) -> Result<()> {

    let mut is_public = false;

    while let Some(token) = tokens.pop() {
        match token.value {
            Token::Keyword(Keyword::Pub) => {
                is_public = true;
                continue;
            },
            Token::Keyword(Keyword::Static) => {
                todo!();
            },
            _ if !is_public => {
                tokens.push(token);
                return Ok(());
            },
            _ => todo!(),
        };

        is_public = false;
    }

    return Ok(());
}

