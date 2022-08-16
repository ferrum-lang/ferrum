use super::*;

use super::super::ast::{self, *};

use super::super::super::lexer::{self, *};

use anyhow::Result;

pub fn build_type(tokens: &mut Stack<TokenData>) -> Result<ast::Type> {
    let mut r#type = match tokens.pop() {
        Some(TokenData { value: Token::BuiltInType(built_in), .. }) => match built_in {
            lexer::BuiltInType::Bool => ast::Type::BuiltIn(TypeBuiltIn::Bool),

            lexer::BuiltInType::Bit => ast::Type::BuiltIn(TypeBuiltIn::Bit),
            lexer::BuiltInType::Byte => ast::Type::BuiltIn(TypeBuiltIn::Byte),

            lexer::BuiltInType::Uint => ast::Type::BuiltIn(TypeBuiltIn::Uint),
            lexer::BuiltInType::Uint8 => ast::Type::BuiltIn(TypeBuiltIn::Uint8),
            lexer::BuiltInType::Uint16 => ast::Type::BuiltIn(TypeBuiltIn::Uint16),
            lexer::BuiltInType::Uint32 => ast::Type::BuiltIn(TypeBuiltIn::Uint32),
            lexer::BuiltInType::Uint64 => ast::Type::BuiltIn(TypeBuiltIn::Uint64),
            lexer::BuiltInType::Uint128 => ast::Type::BuiltIn(TypeBuiltIn::Uint128),
            lexer::BuiltInType::BigUint => ast::Type::BuiltIn(TypeBuiltIn::BigUint),

            lexer::BuiltInType::Int => ast::Type::BuiltIn(TypeBuiltIn::Int),
            lexer::BuiltInType::Int8 => ast::Type::BuiltIn(TypeBuiltIn::Int8),
            lexer::BuiltInType::Int16 => ast::Type::BuiltIn(TypeBuiltIn::Int16),
            lexer::BuiltInType::Int32 => ast::Type::BuiltIn(TypeBuiltIn::Int32),
            lexer::BuiltInType::Int64 => ast::Type::BuiltIn(TypeBuiltIn::Int64),
            lexer::BuiltInType::Int128 => ast::Type::BuiltIn(TypeBuiltIn::Int128),
            lexer::BuiltInType::BigInt => ast::Type::BuiltIn(TypeBuiltIn::BigInt),

            lexer::BuiltInType::Float => ast::Type::BuiltIn(TypeBuiltIn::Float),
            lexer::BuiltInType::Float32 => ast::Type::BuiltIn(TypeBuiltIn::Float32),
            lexer::BuiltInType::Float64 => ast::Type::BuiltIn(TypeBuiltIn::Float64),

            lexer::BuiltInType::Char => ast::Type::BuiltIn(TypeBuiltIn::Char),

            lexer::BuiltInType::String => ast::Type::BuiltIn(TypeBuiltIn::String),

            // _ => todo!(),
        },
        Some(token) if token.value == Token::OpenParenthesis => {
            tokens.push(token);
            build_tuple_type(tokens)?
        },
        Some(token) if token.value == Token::OpenBracket => {
            tokens.push(token);
            build_list_type(tokens)?
        },
        Some(TokenData { value: Token::Identifier(ident), source_meta }) => {
            tokens.push(TokenData { value: Token::Identifier(ident), source_meta });
            ast::Type::Custom(build_custom_type(tokens, None)?)
        },
        Some(TokenData { value: Token::ExclamationMark, .. }) => {
            ast::Type::Result(None)
        },
        Some(token) => todo!("{token:?}"),
        _ => todo!(),
        // None => Err(ParseError::MissingExpectedToken(None))?,
    };

    loop {
        match tokens.peek() {
            Some(TokenData { value: Token::QuestionMark, .. }) => {
                tokens.pop();
                r#type = Type::Optional(Some(Box::new(r#type)));
            },
            Some(TokenData { value: Token::ExclamationMark, .. }) => {
                tokens.pop();
                r#type = Type::Result(Some(Box::new(r#type)));
            },
            _ => break,
        }
    }

    return Ok(r#type);
}

fn build_custom_type(tokens: &mut Stack<TokenData>, receiver: Option<TypeCustom>) -> Result<ast::TypeCustom> {
    let mut r#type = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => match receiver {
            Some(receiver) => TypeCustom {
                name: ident,
                receiver: Some(Box::new(receiver)),
                generics: vec![],
            },
            None => TypeCustom {
                name: ident,
                receiver: None,
                generics: vec![],
            },
        },
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    match tokens.peek() {
        Some(TokenData { value: Token::LessThan, .. }) => {
            tokens.pop();

            loop {
                let generic_type = build_type(tokens)?;

                r#type.generics.push(Box::new(generic_type));

                match tokens.pop() {
                    Some(TokenData { value: Token::Comma, .. }) => {},
                    Some(TokenData { value: Token::GreaterThan, .. }) => break,
                    Some(token) => Err(ParseError::UnexpectedToken(token))?,
                    None => Err(ParseError::MissingExpectedToken(Some(Token::Comma)))?,
                }
            }
        },
        _ => {},
    }

    let r#type = match tokens.peek() {
        Some(token) if token.value == Token::DoubleColon => {
            tokens.pop();
            build_custom_type(tokens, Some(r#type))?
        },
        _ => r#type,
    };

    return Ok(r#type);
}

fn build_tuple_type(tokens: &mut Stack<TokenData>) -> Result<ast::Type> {
    match tokens.pop() {
        Some(TokenData { value: Token::OpenParenthesis, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenParenthesis)))?,
    }

    let first_type = build_type(tokens)?;

    let tuple_type = match tokens.pop() {
        Some(TokenData { value: Token::Semicolon, .. }) => {
            let number = match tokens.pop() {
                Some(TokenData { value: Token::Literal(lexer::Literal::Number(value)), .. }) => ast::LiteralNumber { value },
                Some(token) => Err(ParseError::UnexpectedToken(token))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::Semicolon)))?,
            };

            match tokens.pop() {
                Some(TokenData { value: Token::CloseParenthesis, .. }) => {},
                Some(token) => Err(ParseError::UnexpectedToken(token))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
            }

            ast::TypeTuple::Repeated(TypeTupleRepeated {
                r#type: Box::new(first_type),
                count: number,
            })
        },
        Some(TokenData { value: Token::Comma, .. }) => {
            let mut types = vec![Box::new(first_type)];

            loop {
                let r#type = Box::new(build_type(tokens)?);

                types.push(r#type);

                match tokens.pop() {
                    Some(TokenData { value: Token::CloseParenthesis, .. }) => break,
                    Some(TokenData { value: Token::Comma, .. }) => {},
                    Some(token) => Err(ParseError::UnexpectedToken(token))?,
                    None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
                }
            }

            ast::TypeTuple::Explicit(TypeTupleExplicit { types })
        },
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Comma)))?,
    };

    return Ok(ast::Type::Tuple(tuple_type));
}

fn build_list_type(tokens: &mut Stack<TokenData>) -> Result<ast::Type> {
    match tokens.pop() {
        Some(TokenData { value: Token::OpenBracket, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenBracket)))?,
    }

    let r#type = Box::new(build_type(tokens)?);

    match tokens.pop() {
        Some(TokenData { value: Token::CloseBracket, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBracket)))?,
    }

    return Ok(ast::Type::List(TypeList { r#type }));
}


