use super::*;

use super::super::ast::{self, *};

use super::super::super::lexer::{self, *};

use anyhow::Result;

pub fn build_expression(tokens: &mut Stack<TokenData>) -> Result<ast::Expression> {
    ignore_new_lines(tokens);

    let expr = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) =>
            build_expr_from_ident(tokens, ident.to_string())?,
        Some(TokenData { value: Token::Literal(literal), .. }) =>
            build_expr_from_literal(tokens, literal)?,
        Some(token) if token.value == Token::OpenParenthesis => {
            tokens.push(token);
            build_expr_tuple(tokens)?
        },
        token => todo!("{token:?}"),
    };

    let new_line = ignore_new_lines(tokens);

    let expr = match tokens.peek() {
        Some(TokenData { value: Token::Period, .. }) => {
            tokens.pop();

            ignore_new_lines(tokens);

            let ident = match tokens.pop() {
                Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
                Some(token) => Err(ParseError::UnexpectedToken(token))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
            };

            let receiver = expr;

            let expr = build_expr_from_ident(tokens, ident)?;

            add_reciever_to_expr(expr, receiver)?
        },
        _ => expr,
    };

    if let Some(new_line) = new_line {
        tokens.push(new_line);
    }

    return Ok(expr);
}

fn add_reciever_to_expr(expr: Expression, receiver: Expression) -> Result<Expression> {
    let expr = match expr {
        Expression::Reference(Reference::Instance(mut ref_instance)) => {
            ref_instance.receiver = Some(Box::new(receiver));
            Expression::Reference(Reference::Instance(ref_instance))
        },
        Expression::FunctionCall(fn_call) => Expression::MethodCall(MethodCall {
            name: fn_call.name,
            args: fn_call.args,
            reciever: Box::new(receiver),
        }),
        Expression::MethodCall(method_call) => match *method_call.reciever {
            Expression::Reference(Reference::Instance(mut ref_instance)) => {
                ref_instance.receiver = Some(Box::new(receiver));
                Expression::Reference(Reference::Instance(ref_instance))
            },
            _ => todo!(),
        },
        _ => todo!(),
    };

    return Ok(expr);
}

fn build_expr_from_ident(tokens: &mut Stack<TokenData>, ident: String) -> Result<Expression> {
    let new_line = ignore_new_lines(tokens);

    let expr = match tokens.pop() {
        Some(TokenData { value: Token::Period, .. }) => {
            let receiver = Expression::Reference(Reference::Instance(ReferenceInstance {
                name: ident,
                receiver: None,
            }));

            let ident = match tokens.pop() {
                Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
                Some(TokenData { value: Token::Literal(lexer::Literal::Number(value)), .. }) => value, // number for accessing tuple values
                Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
            };

            let expr = build_expr_from_ident(tokens, ident.to_string())?;

            add_reciever_to_expr(expr, receiver)?
        },
        Some(TokenData { value: Token::OpenParenthesis, .. }) => {
            let mut args = vec![];

            loop {
                match tokens.pop() {
                    Some(TokenData { value: Token::CloseParenthesis, .. }) => break,
                    Some(token) => {
                        tokens.push(token);
                        let expr = build_expression(tokens)?;

                        args.push(FunctionCallArg { name: None, value: Box::new(expr) });
                    },
                    None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
                }
            }

            Expression::FunctionCall(FunctionCall {
                name: ident,
                args,
                reciever: None,
            })
        },
        Some(token) => {
            tokens.push(token);
            Expression::Reference(Reference::Instance(ReferenceInstance {
                name: ident,
                receiver: None,
            }))
        },
        None => Expression::Reference(Reference::Instance(ReferenceInstance {
            name: ident,
            receiver: None,
        })),
    };

    if let Some(new_line) = new_line {
        tokens.push(new_line);
    }

    return Ok(expr);
}

fn build_expr_from_literal(tokens: &mut Stack<TokenData>, literal: lexer::Literal) -> Result<Expression> {
    let expr = match literal {
        lexer::Literal::Bool(value) => Expression::Literal(ast::Literal::Bool(value)),

        lexer::Literal::Number(value) => Expression::Literal(ast::Literal::Number(LiteralNumber { value })),
        lexer::Literal::Char(value) => Expression::Literal(ast::Literal::Char(LiteralChar { value })),

        lexer::Literal::PlainString(string) => Expression::Literal(ast::Literal::String(LiteralString::Plain(string))),
        lexer::Literal::TemplateStringStart(start) => {
            let mut parts = vec![];

            loop {
                let expr = build_expression(tokens)?;
                
                match tokens.pop() {
                    Some(TokenData { value: Token::Literal(lexer::Literal::TemplateStringMiddle(mid)), .. }) => {
                        parts.push(TemplateStringPart {
                            expression: Box::new(expr),
                            post_string: mid,
                        });
                    },
                    Some(TokenData { value: Token::Literal(lexer::Literal::TemplateStringEnd(end)), .. }) => {
                        parts.push(TemplateStringPart {
                            expression: Box::new(expr),
                            post_string: end,
                        });
                        break;
                    },
                    Some(token) => Err(ParseError::UnexpectedToken(token))?,
                    None => Err(ParseError::MissingExpectedToken(Some(Token::Literal(lexer::Literal::TemplateStringEnd("".to_string())))))?,
                }
            }

            Expression::Literal(ast::Literal::String(LiteralString::Template(TemplateString {
                start,
                parts,
            })))
        },

        lexer::Literal::Option { is_some } => match is_some {
            true => {
                let value = match tokens.peek() {
                    Some(TokenData { value: Token::OpenParenthesis, .. }) => {
                        tokens.pop();
                        Some(Box::new(build_expression(tokens)?))
                    },
                    _ => None,
                };

                Expression::Option(ExprOption::Some(value))
            },
            false => Expression::Option(ExprOption::None),
        },
        lexer::Literal::Result { is_ok } => {
            let value = match tokens.peek() {
                Some(TokenData { value: Token::OpenParenthesis, .. }) => {
                    tokens.pop();
                    Some(Box::new(build_expression(tokens)?))
                },
                _ => None,
            };

            Expression::Result(ExprResult { is_ok, value })
        },

        _ => todo!(),
    };

    return Ok(expr);
}

fn build_expr_tuple(tokens: &mut Stack<TokenData>) -> Result<Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::OpenParenthesis, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenParenthesis)))?,
    }

    let first_expr = build_expression(tokens)?;

    let tuple = match tokens.pop() {
        Some(TokenData { value: Token::Comma, .. }) => {
            let mut exprs = vec![Box::new(first_expr)];

            loop {
                let expr = build_expression(tokens)?;

                exprs.push(Box::new(expr));

                match tokens.pop() {
                    Some(TokenData { value: Token::Comma, .. }) => {},
                    Some(TokenData { value: Token::CloseParenthesis, .. }) => break,
                    Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
                    None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
                }
            }

            Tuple::Explicit(TupleExplicit { values: exprs })
        },
        Some(TokenData { value: Token::Semicolon, .. }) => {
            let number = match tokens.pop() {
                Some(TokenData { value: Token::Literal(lexer::Literal::Number(value)), .. }) => ast::LiteralNumber { value },
                Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::Literal(lexer::Literal::Number(String::new())))))?,
            };

            match tokens.pop() {
                Some(TokenData { value: Token::CloseParenthesis, .. }) => {},
                Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
            }

            Tuple::Repeated(TupleRepeated {
                value: Box::new(first_expr),
                count: number,
            })
        },
        Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Comma)))?,
    };


    return Ok(Expression::Tuple(tuple));
}

