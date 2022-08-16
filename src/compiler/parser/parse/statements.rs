use super::*;

use super::super::ast::{self, *};

use super::super::super::lexer::{self, *};

use anyhow::Result;

pub fn parse_statement(
    ast: &mut AST,
    tokens: &mut Stack<TokenData>,
) -> Result<()> {
    let statement = build_statement(tokens)?;

    ast.nodes.push(RootNode::Statement(statement));

    return Ok(());
}

pub fn build_statement(tokens: &mut Stack<TokenData>) -> Result<Statement> {
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

    let statement = if is_assignment {
        let assignment = build_assignment(tokens)?;
        Statement::Assignment(assignment)
    } else {
        let expression = build_expression(tokens)?;
        Statement::Expression(expression)
    };

    // match tokens.pop() {
    //     Some(TokenData { value: Token::NewLine, .. }) => {},
    //     None => {},
    //     token => todo!("\n\n{token:?}\n\n"),
    //     // Some(token) => Err(ParseError::UnexpectedToken(token))?,
    // }

    return Ok(statement);
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

pub fn build_assignment_target(tokens: &mut Stack<TokenData>) -> Result<ast::AssignmentTarget> {
    let target = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => ast::AssignmentTarget::Direct(ident),
        Some(TokenData { value: Token::OpenBrace, .. }) => todo!(),
        Some(token) if token.value == Token::OpenParenthesis => {
            tokens.push(token);
            build_destruct_tuple_assignment_target(tokens)?
        },
        Some(TokenData { value: Token::OpenBracket, .. }) => todo!(),
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    return Ok(target);
}

fn build_destruct_tuple_assignment_target(tokens: &mut Stack<TokenData>) -> Result<ast::AssignmentTarget> {
    match tokens.pop() {
        Some(TokenData { value: Token::OpenParenthesis, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenParenthesis)))?,
    }

    let mut items = vec![];

    loop {
        ignore_new_lines(tokens);

        let is_spread = match tokens.peek() {
            Some(TokenData { value: Token::CloseParenthesis, .. }) => {
                tokens.pop();
                break;
            },
            Some(TokenData { value: Token::DoublePeriod, .. }) => true,
            Some(_) => false,
            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
        };

        if is_spread {
            match tokens.pop() {
                Some(TokenData { value: Token::DoublePeriod, .. }) => {},
                Some(token) => Err(ParseError::UnexpectedToken(token))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::DoublePeriod)))?,
            }

            let item = match tokens.peek() {
                Some(TokenData { value: Token::Identifier(ident), .. }) => {
                    let name = ident.to_string();
                    tokens.pop();
                    
                    AssignTrgtDestructTupleItem::SpreadField(AssignTrgtDestructTupleSpreadField {
                        name,
                    })
                },
                _ => AssignTrgtDestructTupleItem::Spread,
            };

            items.push(item);
        } else {
            let inner = build_assignment_target(tokens)?;

            items.push(AssignTrgtDestructTupleItem::Field(AssignTrgtDestructTupleField {
                value: Box::new(inner),
            }));
        }

        match tokens.pop() {
            Some(TokenData { value: Token::Comma, .. }) => {},
            Some(TokenData { value: Token::CloseParenthesis, .. }) => break,
            Some(token) => Err(ParseError::UnexpectedToken(token))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
        }
    }

    return Ok(ast::AssignmentTarget::DestructureTuple(AssignTrgtDestructTuple {
        items,
    }));
}

fn build_assignment_expression(tokens: &mut Stack<TokenData>) -> Result<ast::Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::Equals, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Equals)))?,
    }

    return build_expression(tokens);
}
