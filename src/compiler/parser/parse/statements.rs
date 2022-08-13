use super::*;

use super::super::ast::{self, *};

use super::super::super::lexer::{self, *};

use anyhow::Result;

pub fn parse_statement(
    ast: &mut AST,
    tokens: &mut Stack<TokenData>,
) -> Result<()> {
    let is_assignment = match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::Const), .. }) => true,
        Some(TokenData { value: Token::Keyword(Keyword::Let), .. }) => true,
        Some(TokenData { value: Token::Identifier(_), .. }) => {
            let token = tokens.pop().unwrap();

            let res = match tokens.peek() {
                Some(TokenData { value: Token::Colon, .. }) => true,
                Some(TokenData { value: Token::Equals, .. }) => true,
                _ => false,
            };

            tokens.push(token);

            res
        },
        Some(_) => false,
        _ => todo!(),
        // None => Err(ParseError::MissingExpectedToken(None))?,
    };

    if is_assignment {
        let assignment = build_assignment(tokens)?;
        ast.nodes.push(RootNode::Statement(Statement::Assignment(assignment)));
    } else {
        let expression = build_expression(tokens)?;
        ast.nodes.push(RootNode::Statement(Statement::Expression(expression)));
    }

    match tokens.pop() {
        Some(TokenData { value: Token::NewLine, .. }) => {},
        None => {},
        token => todo!("{token:?}\n{ast:?}"),
        // Some(token) => Err(ParseError::UnexpectedToken(token))?,
    }

    return Ok(());
}

fn build_assignment(tokens: &mut Stack<TokenData>) -> Result<ast::Assignment> {
    let local_var = match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Const), .. }) => Some(AssignmentLocalVar::Const),
        Some(TokenData { value: Token::Keyword(Keyword::Let), .. }) => Some(AssignmentLocalVar::Let),
        Some(token) => {
            tokens.push(token);
            None
        },
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    let target = build_assignment_target(tokens)?;
    
    let explicit_type = match tokens.peek() {
        Some(TokenData { value: Token::Colon, .. }) => {
            tokens.pop();
            Some(build_type(tokens)?)
        },
        _ => None,
    };

    let expression = build_assignment_expression(tokens)?;

    return Ok(ast::Assignment {
        local_var,
        target,
        explicit_type,
        expression,
    });
}

fn build_assignment_target(tokens: &mut Stack<TokenData>) -> Result<ast::AssignmentTarget> {
    let target = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => ast::AssignmentTarget::Direct(ident),
        Some(TokenData { value: Token::OpenBrace, .. }) => todo!(),
        Some(TokenData { value: Token::OpenParenthesis, .. }) => todo!(),
        Some(TokenData { value: Token::OpenBracket, .. }) => todo!(),
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    return Ok(target);
}

fn build_assignment_expression(tokens: &mut Stack<TokenData>) -> Result<ast::Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::Equals, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Equals)))?,
    }

    return build_expression(tokens);
}
