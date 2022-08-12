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
        None => Err(ParseError::MissingExpectedToken(None))?,
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
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(None))?,
    }

    return Ok(());
}

fn build_assignment(tokens: &mut Stack<TokenData>) -> Result<ast::Assignment> {
    todo!();
}

fn build_expression(tokens: &mut Stack<TokenData>) -> Result<ast::Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) =>
            return build_expr_from_ident(tokens, ident.to_string()),
        Some(TokenData { value: Token::Literal(literal), .. }) =>
            return build_expr_from_literal(tokens, literal),
        token => todo!("{token:?}"),
    }
}

fn build_expr_from_ident(tokens: &mut Stack<TokenData>, ident: String) -> Result<Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::Period, .. }) => {
            let receiver = Expression::Reference(Reference::Instance(ReferenceInstance {
                name: ident,
                receiver: None,
            }));

            let ident = match tokens.pop() {
                Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
                Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
            };

            let expr = build_expr_from_ident(tokens, ident.to_string())?;

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
                _ => todo!(),
            };

            return Ok(expr);
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

            let expr = Expression::FunctionCall(FunctionCall {
                name: ident,
                args,
                reciever: None,
            });

            return Ok(expr);
        },
        Some(token) => {
            tokens.push(token);
            return Ok(Expression::Reference(Reference::Instance(ReferenceInstance {
                name: ident,
                receiver: None,
            })));
        },
        None => {
            return Ok(Expression::Reference(Reference::Instance(ReferenceInstance {
                name: ident,
                receiver: None,
            })));
        },
    }
}

fn build_expr_from_literal(tokens: &mut Stack<TokenData>, literal: lexer::Literal) -> Result<Expression> {
    let expr = match literal {
        lexer::Literal::Bool(value) => Expression::Literal(ast::Literal::Bool(value)),
        // lexer::Literal::Char(value) => Expression::Literal(ast::Literal::Char()), // TODO
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
        _ => todo!(),
    };

    return Ok(expr);
}


