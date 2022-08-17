use super::*;

use super::super::ast::{self, *};

use super::super::super::lexer::{self, *};

use anyhow::Result;

pub fn parse_static_consts(
    ast: &mut AST,
    tokens: &mut Stack<TokenData>,
) -> Result<()> {

    let mut public_token = None;

    while let Some(token) = tokens.pop() {
        match token.value {
            Token::Keyword(Keyword::Pub) => {
                public_token = Some(token);
                continue;
            },
            Token::Keyword(Keyword::Static) => {
                todo!();
            },
            _ => {
                tokens.push(token);

                if let Some(public_token) = public_token {
                    tokens.push(public_token);
                }

                return Ok(());
            },
            _ => todo!(),
        };

        public_token = None;
    }

    return Ok(());
}

