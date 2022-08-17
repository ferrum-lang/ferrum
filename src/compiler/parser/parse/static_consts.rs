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
                tokens.push(token);
                let static_const = build_static_const(tokens, public_token.is_some())?;
                ast.static_consts.push(static_const);
            },
            _ => {
                tokens.push(token);

                if let Some(public_token) = public_token {
                    tokens.push(public_token);
                }

                return Ok(());
            },
        };

        public_token = None;
    }

    return Ok(());
}

pub fn build_static_const(tokens: &mut Stack<TokenData>, is_public: bool) -> Result<StaticConst> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Static), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Static))))?,
    }

    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Const), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Const))))?,
    }

    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(name), .. }) => name,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    ignore_new_lines(tokens);

    let r#type = match tokens.peek() {
        Some(TokenData { value: Token::Colon, .. }) => Some(build_type(tokens)?),
        _ => None,
    };

    ignore_new_lines(tokens);

    match tokens.pop() {
        Some(TokenData { value: Token::Equals, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Equals)))?,
    }

    let value = build_expression(tokens)?;

    return Ok(StaticConst {
        name,
        r#type,
        value,
        is_public,
    });
}

