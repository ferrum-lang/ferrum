use super::*;

use super::super::ast::{self, *};

use super::super::super::lexer::{self, *};

use anyhow::Result;

const SOME_STR: &'static str = "some";
const NONE_STR: &'static str = "none";

const OK_STR: &'static str = "ok";
const ERR_STR: &'static str = "err";

pub fn build_pattern(tokens: &mut Stack<TokenData>) -> Result<Pattern> {
    let pattern = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => {
            build_pattern_from_ident(tokens, ident, None)?
        },
        Some(TokenData { value: Token::Literal(literal), .. }) => {
            build_pattern_from_literal(tokens, literal)?
        },
        Some(TokenData { value: Token::OpenParenthesis, .. }) => todo!(),
        Some(TokenData { value: Token::OpenBracket, .. }) => todo!(),
        Some(TokenData { value: Token::OpenBrace, .. }) => todo!(),
        token => todo!("{token:?}"),
    };

    return Ok(pattern);
}

fn build_pattern_from_ident(
    tokens: &mut Stack<TokenData>,
    ident: String,
    receiver: Option<PatternIdentity>,
) -> Result<Pattern> {
    let pattern = match tokens.peek() {
        Some(TokenData { value: Token::OpenParenthesis, .. }) => {
            Pattern::TupleStruct(build_pattern_tuple_struct(tokens, ident, receiver)?)
        },
        Some(TokenData { value: Token::OpenBrace, .. }) => {
            tokens.pop();
            todo!();
        },
        Some(TokenData { value: Token::DoubleColon, .. }) => {
           tokens.pop();

           let receiver = Some(PatternIdentity {
                name: ident,
                receiver: receiver.map(Box::new),
           });

           let ident = match tokens.pop() {
                Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
                Some(token) => Err(ParseError::UnexpectedToken(token))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
           };

           build_pattern_from_ident(tokens, ident, receiver)?
        },
        _ => Pattern::Identity(PatternIdentity {
            name: ident,
            receiver: receiver.map(Box::new),
        }),
    };

    return Ok(pattern);
}

fn build_pattern_from_literal(tokens: &mut Stack<TokenData>, literal: lexer::Literal) -> Result<Pattern> {
    let pattern = match literal {
        lexer::Literal::Option { is_some } => build_pattern_from_option(tokens, is_some)?,
        lexer::Literal::Result { is_ok } => build_pattern_from_result(tokens, is_ok)?,
        lexer::Literal::Bool(value) => Pattern::Literal(PatternLiteral::Bool(value)),
        lexer::Literal::Char(value) => Pattern::Literal(PatternLiteral::Char(value)),
        lexer::Literal::Number(value) => Pattern::Literal(PatternLiteral::Number(value)),
        lexer::Literal::PlainString(value) => Pattern::Literal(PatternLiteral::String(value)),
        _ => todo!(),
    };

    return Ok(pattern);
}

fn build_pattern_from_option(tokens: &mut Stack<TokenData>, is_some: bool) -> Result<Pattern> {
    if !is_some {
        return Ok(Pattern::TupleStruct(PatternTupleStruct {
            name: NONE_STR.to_string(),
            args: vec![],
            receiver: None,
        }));
    }

    let pattern = match tokens.peek() {
        Some(TokenData { value: Token::OpenParenthesis, .. }) => {
            Pattern::TupleStruct(build_pattern_tuple_struct(tokens, SOME_STR.to_string(), None)?)
        },
        _ => Pattern::TupleStruct(PatternTupleStruct {
            name: SOME_STR.to_string(),
            args: vec![],
            receiver: None,
        }),
    };

    return Ok(pattern);
}

fn build_pattern_from_result(tokens: &mut Stack<TokenData>, is_ok: bool) -> Result<Pattern> {
    let name = if is_ok { OK_STR } else { ERR_STR }.to_string();

    let pattern = match tokens.peek() {
        Some(TokenData { value: Token::OpenParenthesis, .. }) => {
            Pattern::TupleStruct(build_pattern_tuple_struct(tokens, name, None)?)
        },
        _ => Pattern::TupleStruct(PatternTupleStruct {
            name,
            args: vec![],
            receiver: None,
        }),
    };

    return Ok(pattern);
}

fn build_pattern_tuple_struct(tokens: &mut Stack<TokenData>, name: String, receiver: Option<PatternIdentity>) -> Result<PatternTupleStruct> {
    match tokens.pop() {
        Some(TokenData { value: Token::OpenParenthesis, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenParenthesis)))?,
    }

    let mut args = vec![];

    loop {
        ignore_new_lines(tokens);

        match tokens.peek() {
            Some(TokenData { value: Token::CloseParenthesis, .. }) => {
                tokens.pop();
                break;
            },
            _ => {},
        }

        args.push(Box::new(build_pattern(tokens)?));

        ignore_new_lines(tokens);

        match tokens.pop() {
            Some(TokenData { value: Token::CloseParenthesis, .. }) => break,
            Some(TokenData { value: Token::Comma, .. }) => {},
            Some(token) => Err(ParseError::UnexpectedToken(token))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
        }
    }

    return Ok(PatternTupleStruct { name, args, receiver })
}

