use super::*;

use super::super::ast::AST;

use anyhow::Result;

pub fn parse_definition(
    ast: &mut AST,
    tokens: &mut Stack<TokenData>,
) -> Result<()> {
    let definition = build_definition(tokens)?;

    ast.nodes.push(RootNode::Definition(definition));

    return Ok(());
}

pub fn build_definition(tokens: &mut Stack<TokenData>) -> Result<Definition> {
    let is_public = match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::Pub), .. }) => {
            tokens.pop();
            true
        },
        _ => false,
    };

    let definition = match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::Fn), .. }) => build_definition_fn(tokens, is_public)?,
        _ => todo!(),
        // None => Err(ParseError::MissingExpectedToken(None))?,
    };

    match tokens.pop() {
        Some(TokenData { value: Token::NewLine, .. }) => {},
        None => {},
        token => todo!("\n\n{token:?}\n\n"),
        // Some(token) => Err(ParseError::UnexpectedToken(token))?,
    }

    return Ok(definition);
}

fn build_definition_fn(tokens: &mut Stack<TokenData>, is_public: bool) -> Result<Definition> {
    let signature = build_def_fn_signature(tokens, is_public)?;

    ignore_new_lines(tokens);
    
    let r#impl = match tokens.pop() {
        Some(TokenData { value: Token::FatArrow, .. }) => DefFnImpl::Expression(build_expression(tokens)?),
        Some(token) if token.value == Token::OpenBrace => {
            tokens.push(token);
            DefFnImpl::Block(build_statement_block(tokens)?)
        },
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenBrace)))?,
    };

    return Ok(Definition::Function(DefFn {
        r#impl,
        signature,
    }));
}

fn build_def_fn_signature(tokens: &mut Stack<TokenData>, is_public: bool) -> Result<DefFnSignature> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Fn), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Fn))))?,
    }

    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    let generics = match tokens.peek() {
        Some(TokenData { value: Token::LessThan, .. }) => todo!(),
        _ => None,
    };

    match tokens.pop() {
        Some(TokenData { value: Token::OpenParenthesis, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenParenthesis)))?,
    }

    let mut params = vec![];

    loop {
        ignore_new_lines(tokens);

        match tokens.peek() {
            Some(TokenData { value: Token::CloseParenthesis, .. }) => {
                tokens.pop();
                break;
            },
            _ => {},
        }

        ignore_new_lines(tokens);

        let param = build_def_fn_param(tokens)?;
        params.push(param);

        match tokens.pop() {
            Some(TokenData { value: Token::CloseParenthesis, .. }) => break,
            Some(TokenData { value: Token::Comma, .. }) => {},
            Some(token) => Err(ParseError::UnexpectedToken(token))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
        }
    }

    let return_type = match tokens.peek() {
        Some(TokenData { value: Token::SkinnyArrow, .. }) => {
            tokens.pop();
            Some(build_type(tokens)?)
        },
        _ => None,
    };

    return Ok(DefFnSignature {
        is_public,
        name,
        generics,
        params,
        return_type,
    });
}

fn build_def_fn_param(tokens: &mut Stack<TokenData>) -> Result<DefFnParam> {
    let is_var_args = match tokens.peek() {
        Some(TokenData { value: Token::DoublePeriod, .. }) => {
            tokens.pop();
            true
        },
        _ => false,
    };

    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    ignore_new_lines(tokens);

    match tokens.pop() {
        Some(TokenData { value: Token::Colon, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Colon)))?,
    }

    let is_mutable = match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::Mut), .. }) => {
            tokens.pop();
            true
        },
        _ => false,
    };

    let r#type = build_type(tokens)?;

    ignore_new_lines(tokens);

    let r#default = match tokens.peek() {
        Some(TokenData { value: Token::Equals, .. }) => Some(Box::new(build_expression(tokens)?)),
        _ => None,
    };

    return Ok(DefFnParam {
        name,
        r#type,
        is_mutable,
        is_var_args,
        default,
    });
}

