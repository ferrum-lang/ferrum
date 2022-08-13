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
        token => todo!("{token:?}"),
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

fn build_type(tokens: &mut Stack<TokenData>) -> Result<ast::Type> {
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

fn build_assignment_expression(tokens: &mut Stack<TokenData>) -> Result<ast::Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::Equals, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Equals)))?,
    }

    return build_expression(tokens);
}

fn build_expression(tokens: &mut Stack<TokenData>) -> Result<ast::Expression> {
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
    ignore_new_lines(tokens);

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

            let expr = add_reciever_to_expr(expr, receiver)?;

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

