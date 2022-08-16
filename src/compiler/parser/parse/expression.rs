use super::*;

use super::super::ast::{self, *};

use super::super::super::lexer::{self, *};

use anyhow::Result;

pub fn build_expression(tokens: &mut Stack<TokenData>) -> Result<ast::Expression> {
    ignore_new_lines(tokens);

    match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::Mut), .. }) => {
            tokens.pop();
            let inner = Box::new(build_expression(tokens)?);
            return Ok(ast::Expression::Mut(inner));
        },
        Some(TokenData { value: Token::Colon, .. }) => {
            tokens.pop();
            let inner = Box::new(build_expression(tokens)?);
            return Ok(ast::Expression::TernaryElse(inner));
        },
        _ => {},
    }

    let expr = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => {
            build_expr_from_ident(tokens, ident.to_string())?
        },
        Some(TokenData { value: Token::Literal(literal), .. }) => {
            build_expr_from_literal(tokens, literal)?
        },
        Some(TokenData { value: Token::ExclamationMark, .. }) => {
            Expression::Result(ExprResult::Passed(ExprResultPassed {
                reciever: None,
            }))
        },
        Some(token) if token.value == Token::OpenParenthesis => {
            tokens.push(token);
            build_expr_in_parens(tokens)?
        },
        Some(token) if token.value == Token::OpenBracket => {
            tokens.push(token);
            build_expr_list(tokens)?
        },
        Some(token) if token.value == Token::OpenBrace => {
            tokens.push(token);
            let block = Box::new(build_statement_block(tokens)?);
            ast::Expression::Block(BlockExpr { block })
        },
        Some(TokenData { value: Token::Keyword(keyword), source_meta }) => {
            tokens.push(TokenData { value: Token::Keyword(keyword), source_meta });
            build_expr_keyword(tokens)?
        },
        token => todo!("{token:?}"),
    };

    let expr = wrap_expression(tokens, expr)?;

    let new_line = ignore_new_lines(tokens);

    let expr = build_binary_operation_from(tokens, expr)?;

    if let Some(new_line) = new_line {
        tokens.push(new_line);
    }

    return Ok(expr);
}

fn wrap_expression(tokens: &mut Stack<TokenData>, expr: Expression) -> Result<Expression> {
    let mut expr = expr;

    loop {
        expr = match tokens.peek() {
            Some(TokenData { value: Token::QuestionMark, .. }) => {
                tokens.pop();

                let mut tokens_clone = tokens.clone();

                let ternary_if = build_expression(&mut tokens_clone);

                match ternary_if {
                    Err(_) => Expression::Option(ExprOption::Passed(ExprOptionPassed {
                        reciever: Box::new(expr),
                    })),
                    Ok(ternary_if) => {
                        match tokens_clone.pop() {
                            Some(TokenData { value: Token::Colon, .. }) => {
                                *tokens = tokens_clone;
                                let ternary_else = build_expression(tokens)?;

                                Expression::Branch(Branch::Ternary(BranchTernary {
                                    condition: Box::new(expr),
                                    if_then: Box::new(ternary_if),
                                    else_then: Box::new(ternary_else),
                                }))
                            },
                            _ => Expression::Option(ExprOption::Passed(ExprOptionPassed {
                                reciever: Box::new(expr),
                            })),
                        }
                    }
                }
            },
            Some(TokenData { value: Token::ExclamationMark, .. }) => {
                tokens.pop();

                Expression::Result(ExprResult::Passed(ExprResultPassed {
                    reciever: Some(Box::new(expr)),
                }))
            },
            Some(TokenData { value: Token::OpenBracket, .. }) => {
                tokens.pop();

                let index = build_expression(tokens)?;

                match tokens.pop() {
                    Some(TokenData { value: Token::CloseBracket, .. }) => {},
                    Some(token) => Err(ParseError::UnexpectedToken(token))?,
                    None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBracket)))?,
                }

                Expression::ListIndexedItem(ListIndexedItem {
                    receiver: Box::new(expr),
                    index: Box::new(index),
                })
            },
            _ => break,
        };
    }

    return Ok(expr);
}

fn build_binary_operation_from(tokens: &mut Stack<TokenData>, expr: Expression) -> Result<Expression> {
    let mut expr = expr;

    loop {
        let new_line = ignore_new_lines(tokens);

        expr = match tokens.peek() {
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
            Some(TokenData { value: Token::Plus, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::Plus,
                })
            },
            Some(TokenData { value: Token::Minus, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::Minus,
                })
            },
            Some(TokenData { value: Token::Asterisk, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::Multiply,
                })
            },
            Some(TokenData { value: Token::ForwardSlash, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::Divide,
                })
            },
            Some(TokenData { value: Token::Percent, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::Mod,
                })
            },
            Some(TokenData { value: Token::Caret, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::Pow,
                })
            },
            Some(TokenData { value: Token::PlusEquals, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::PlusEquals,
                })
            },
            Some(TokenData { value: Token::MinusEquals, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::MinusEquals,
                })
            },
            Some(TokenData { value: Token::AsteriskEquals, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::MultiplyEquals,
                })
            },
            Some(TokenData { value: Token::ForwardSlashEquals, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::DivideEquals,
                })
            },
            Some(TokenData { value: Token::PercentEquals, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::ModEquals,
                })
            },
            Some(TokenData { value: Token::CaretEquals, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::PowEquals,
                })
            },
            Some(TokenData { value: Token::DoubleEquals, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::Equals,
                })
            },
            Some(TokenData { value: Token::NotEquals, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::NotEquals,
                })
            },
            Some(TokenData { value: Token::GreaterThan, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::GreaterThan,
                })
            },
            Some(TokenData { value: Token::GreaterThanEquals, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::GreaterThanOrEquals,
                })
            },
            Some(TokenData { value: Token::LessThan, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::LessThan,
                })
            },
            Some(TokenData { value: Token::LessThanEquals, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::LessThanOrEquals,
                })
            },
            Some(TokenData { value: Token::DoublePipe, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::Or,
                })
            },
            Some(TokenData { value: Token::DoubleAmpersand, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::And,
                })
            },
            Some(TokenData { value: Token::DoubleQuestionMark, .. }) => {
                tokens.pop();

                let left = Box::new(expr);

                let right = Box::new(build_expression(tokens)?);

                Expression::BinaryOperation(BinaryOperation {
                    left,
                    right,
                    operator: BinaryOperator::NullCoalesce
                })
            },
            Some(TokenData { value: Token::Keyword(Keyword::Matches), .. }) => {
                tokens.pop();

                let value = Box::new(expr);

                let pattern = build_pattern(tokens)?;

                Expression::Matches(Matches {
                    value,
                    pattern,
                })
            },
            Some(TokenData { value: Token::DoublePeriod, .. }) => {
                tokens.pop();

                let from = Box::new(expr);

                let to = Box::new(build_expression(tokens)?);

                Expression::Range(Range {
                    from,
                    to,
                    inclusive: false,
                })
            },
            Some(TokenData { value: Token::DoublePeriodEquals, .. }) => {
                tokens.pop();

                let from = Box::new(expr);

                let to = Box::new(build_expression(tokens)?);

                Expression::Range(Range {
                    from,
                    to,
                    inclusive: true,
                })
            },
            _ => {
                if let Some(new_line) = new_line {
                    tokens.push(new_line);
                }
                break;
            },
        };
    }

    return Ok(expr);
}

fn add_reciever_to_expr(expr: Expression, receiver: Expression) -> Result<Expression> {
    let expr = match expr {
        Expression::Reference(Reference::Instance(mut ref_instance)) => {
            ref_instance.receiver = Some(Box::new(receiver));
            Expression::Reference(Reference::Instance(ref_instance))
        },
        Expression::Reference(Reference::Static(mut ref_static)) => {
            ref_static.receiver = Some(Box::new(match receiver {
                Expression::Reference(Reference::Static(receiver)) => receiver,
                _ => todo!(),
            }));

            Expression::Reference(Reference::Static(ref_static))
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
                ignore_new_lines(tokens);

                match tokens.peek() {
                    Some(TokenData { value: Token::CloseParenthesis, .. }) => {
                        tokens.pop();
                        break;
                    },
                    _ => {},
                }

                ignore_new_lines(tokens);

                let value = Box::new(build_expression(tokens)?);
                args.push(FunctionCallArg { name: None, value });

                match tokens.pop() {
                    Some(TokenData { value: Token::CloseParenthesis, .. }) => break,
                    Some(TokenData { value: Token::Comma, .. }) => {},
                    Some(token) => Err(ParseError::UnexpectedToken(token))?,
                    None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
                }
            }

            Expression::FunctionCall(FunctionCall {
                name: ident,
                args,
                reciever: None,
            })
        },
        Some(TokenData { value: Token::DoubleColon, .. }) => {
            let receiver = Expression::Reference(Reference::Static(ReferenceStatic {
                name: ident,
                receiver: None,
            }));

            let ident = match tokens.pop() {
                Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
                Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
            };

            let expr = build_expr_from_ident(tokens, ident.to_string())?;

            add_reciever_to_expr(expr, receiver)?
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
                        
                        let value = Some(Box::new(build_expression(tokens)?));

                        match tokens.pop() {
                            Some(TokenData { value: Token::CloseParenthesis, .. }) => {},
                            Some(token) => Err(ParseError::UnexpectedToken(token))?,
                            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
                        }

                        value
                    },
                    _ => None,
                };

                Expression::Option(ExprOption::Direct(ExprOptionDirect::Some(value)))
            },
            false => Expression::Option(ExprOption::Direct(ExprOptionDirect::None)),
        },
        lexer::Literal::Result { is_ok } => {
            let value = match tokens.peek() {
                Some(TokenData { value: Token::OpenParenthesis, .. }) => {
                    tokens.pop();

                    let value = match tokens.peek() {
                        Some(TokenData { value: Token::CloseParenthesis, .. }) => {
                            tokens.pop();
                            None
                        },
                        Some(_) => {
                            let value = Box::new(build_expression(tokens)?);
                            match tokens.pop() {
                                Some(TokenData { value: Token::CloseParenthesis, .. }) => {},
                                Some(token) => Err(ParseError::UnexpectedToken(token))?,
                                None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
                            }

                            Some(value)
                        },
                        None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
                    };

                    value
                },
                _ => None,
            };

            Expression::Result(ExprResult::Direct(ExprResultDirect { is_ok, value }))
        },

        _ => todo!(),
    };

    return Ok(expr);
}

fn build_expr_keyword(tokens: &mut Stack<TokenData>) -> Result<Expression> {
    let expr = match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::If), .. }) => build_expr_if_else(tokens)?,
        Some(TokenData { value: Token::Keyword(Keyword::Match), .. }) => build_expr_match(tokens)?,
        Some(TokenData { value: Token::Keyword(Keyword::Loop), .. }) => build_expr_loop(tokens)?,
        Some(TokenData { value: Token::Keyword(Keyword::While), .. }) => build_expr_while(tokens)?,
        Some(TokenData { value: Token::Keyword(Keyword::For), .. }) => build_expr_for(tokens)?,
        token => todo!("{token:?}"),
    };

    return Ok(expr);
}

fn build_expr_if_else(tokens: &mut Stack<TokenData>) -> Result<Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::If), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::If))))?,
    }

    let condition = Box::new(build_expression(tokens)?);
    let then = Box::new(build_statement_block(tokens)?);

    let new_line = ignore_new_lines(tokens);

    let r#else = match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::Else), .. }) => {
            tokens.pop();
            ignore_new_lines(tokens);
            Some(Box::new(build_expression(tokens)?))
        },
        _ => None,
    };

    if let Some(new_line) = new_line {
        tokens.push(new_line);
    }

    return Ok(Expression::Branch(Branch::IfElse(BranchIfElse {
        condition,
        then,
        r#else,
    })));
}

fn build_expr_match(tokens: &mut Stack<TokenData>) -> Result<Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Match), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Match))))?,
    }

    let value = Box::new(build_expression(tokens)?);

    ignore_new_lines(tokens);

    match tokens.pop() {
        Some(TokenData { value: Token::OpenBrace, .. }) => {}
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBrace)))?,
    }

    let mut arms = vec![];

    loop {
        ignore_new_lines(tokens);

        match tokens.peek() {
            Some(TokenData { value: Token::CloseBrace, .. }) => {
                tokens.pop();
                break;
            },
            _ => {},
        }

        ignore_new_lines(tokens);

        let arm = build_match_arm(tokens)?;
        arms.push(arm);

        match tokens.pop() {
            Some(TokenData { value: Token::CloseBrace, .. }) => break,
            Some(TokenData { value: Token::Comma, .. }) => {},
            Some(token) => Err(ParseError::UnexpectedToken(token))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBrace)))?,
        }
    }

    return Ok(Expression::Branch(Branch::Match(BranchMatch {
        value,
        arms,
    })));
}

fn build_match_arm(tokens: &mut Stack<TokenData>) -> Result<BranchMatchArm> {
    let pattern = build_pattern(tokens)?;

    let guard = match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::If), .. }) => {
            tokens.pop();
            
            Some(Box::new(build_expression(tokens)?))
        },
        _ => None,
    };

    match tokens.pop() {
        Some(TokenData { value: Token::FatArrow, .. }) => {}
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::FatArrow)))?,
    }

    let body = match tokens.peek() {
        Some(TokenData { value: Token::OpenBrace, .. }) => Box::new(build_statement_block(tokens)?),
        _ => Box::new(Block { statements: vec![build_statement(tokens)?] }),
    };

    return Ok(BranchMatchArm {
        pattern,
        guard,
        body,
    });
}

fn build_expr_loop(tokens: &mut Stack<TokenData>) -> Result<Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Loop), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Loop))))?,
    }

    let block = Box::new(build_statement_block(tokens)?);

    let post_while_condition = match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::While), .. }) => {
            tokens.pop();

            Some(Box::new(build_expression(tokens)?))
        },
        _ => None,
    };

    return Ok(Expression::Loop(Loop::Loop(LoopLoop {
        block,
        post_while_condition,
    })));
}

fn build_expr_while(tokens: &mut Stack<TokenData>) -> Result<Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::While), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::While))))?,
    }

    let condition = Box::new(build_expression(tokens)?);

    let block = Box::new(build_statement_block(tokens)?);

    return Ok(Expression::Loop(Loop::While(LoopWhile {
        condition,
        block,
    })));
}

fn build_expr_for(tokens: &mut Stack<TokenData>) -> Result<Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::For), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::For))))?,
    }

    let item = build_assignment_target(tokens)?;

    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::In), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::In))))?,
    }

    let expression = Box::new(build_expression(tokens)?);

    let block = Box::new(build_statement_block(tokens)?);

    return Ok(Expression::Loop(Loop::For(LoopFor {
        item,
        expression,
        block,
    })));
}

fn build_statement_block(tokens: &mut Stack<TokenData>) -> Result<Block> {
    match tokens.pop() {
        Some(TokenData { value: Token::OpenBrace, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::If))))?,
    }

    let mut statements = vec![];

    loop {
        ignore_new_lines(tokens);

        match tokens.peek() {
            Some(token) if token.value == Token::CloseBrace => {
                tokens.pop();
                break;
            },
            Some(_) => statements.push(build_statement(tokens)?),
            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBrace)))?,
        }
    }

    return Ok(Block { statements });
}

fn build_expr_in_parens(tokens: &mut Stack<TokenData>) -> Result<Expression> {
    let mut stack = Stack::new();

    let mut is_closure = false;

    let mut inner_count = 0;
    loop {
        match tokens.pop() {
            Some(token) if token.value == Token::OpenParenthesis => {
                inner_count += 1;
                stack.push(token);
            },
            Some(token) if token.value == Token::CloseParenthesis => {
                inner_count -= 1;
                stack.push(token);

                if inner_count <= 0 {
                    let new_line = ignore_new_lines(tokens);

                    match tokens.pop() {
                        Some(token) if token.value == Token::SkinnyArrow || token.value == Token::FatArrow => {
                            tokens.push(token);
                            is_closure = true;
                        },
                        Some(token) => {
                            tokens.push(token);
                        },
                        None => {}
                    }

                    if let Some(new_line) = new_line {
                        tokens.push(new_line);
                    }

                    while let Some(token) = stack.pop() {
                        tokens.push(token);
                    }

                    break;
                }
            },
            Some(token) => {
                stack.push(token);
            },
            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
        }
    }

    if is_closure {
        return build_expr_closure(tokens);
    } else {
        return build_expr_tuple(tokens);
    }
}

fn build_expr_closure(tokens: &mut Stack<TokenData>) -> Result<Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::OpenParenthesis, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenParenthesis)))?,
    }

    let mut params = vec![];

    loop {
        match tokens.peek() {
            Some(TokenData { value: Token::CloseParenthesis, .. }) => {
                tokens.pop();
                break;
            },
            _ => {},
        }

        let param = build_closure_param(tokens)?;

        params.push(param);
    }

    let return_type = match tokens.peek() {
        Some(TokenData { value: Token::Colon, .. }) => {
            tokens.pop();
            Some(build_type(tokens)?)
        },
        _ => None,
    };

    let signature = ClosureSignature {
        params,
        return_type,
    };

    match tokens.pop() {
        Some(TokenData { value: Token::FatArrow, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::FatArrow)))?,
    }

    let r#impl = Box::new(build_expression(tokens)?);

    return Ok(Expression::Closure(Closure {
        signature,
        r#impl,
    }));
}

fn build_closure_param(tokens: &mut Stack<TokenData>) -> Result<ClosureParam> {
    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
        Some(TokenData { value: Token::OpenBrace, .. }) => todo!(),
        Some(TokenData { value: Token::OpenParenthesis, .. }) => todo!(),
        Some(TokenData { value: Token::OpenBracket, .. }) => todo!(),
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    let r#type = match tokens.peek() {
        Some(TokenData { value: Token::Colon, .. }) => {
            tokens.pop();
            Some(build_type(tokens)?)
        },
        _ => None,
    };

    return Ok(ClosureParam {
        name,
        r#type,
    });
}

fn build_expr_tuple(tokens: &mut Stack<TokenData>) -> Result<Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::OpenParenthesis, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenParenthesis)))?,
    }

    ignore_new_lines(tokens);

    let first_expr = build_expression(tokens)?;

    ignore_new_lines(tokens);

    let tuple = match tokens.pop() {
        Some(TokenData { value: Token::CloseParenthesis, .. }) => Tuple::Explicit(TupleExplicit {
            values: vec![Box::new(first_expr)],
        }),
        Some(TokenData { value: Token::Comma, .. }) => {
            let mut exprs = vec![Box::new(first_expr)];

            loop {
                ignore_new_lines(tokens);

                match tokens.peek() {
                    Some(TokenData { value: Token::CloseParenthesis, .. }) => {
                        tokens.pop();
                        break;
                    },
                    Some(_) => {},
                    None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
                }

                let expr = build_expression(tokens)?;

                exprs.push(Box::new(expr));

                ignore_new_lines(tokens);

                match tokens.pop() {
                    Some(TokenData { value: Token::Comma, .. }) => {},
                    Some(TokenData { value: Token::CloseParenthesis, .. }) => break,
                    Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
                    None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
                }

                ignore_new_lines(tokens);
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
        None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
    };

    return Ok(Expression::Tuple(tuple));
}

fn build_expr_list(tokens: &mut Stack<TokenData>) -> Result<Expression> {
    match tokens.pop() {
        Some(TokenData { value: Token::OpenBracket, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenBracket)))?,
    }

    ignore_new_lines(tokens);

    let first_expr = match tokens.peek() {
        Some(TokenData { value: Token::DoublePeriod, .. }) => {
            tokens.pop();
            Expression::ListValueSpread(Box::new(build_expression(tokens)?))
        },
        _ => build_expression(tokens)?,
    };

    ignore_new_lines(tokens);

    let list = match tokens.pop() {
        Some(TokenData { value: Token::CloseBracket, .. }) => List::Explicit(ListExplicit { values: vec![Box::new(first_expr)] }),
        Some(TokenData { value: Token::Comma, .. }) => {
            let mut exprs = vec![Box::new(first_expr)];

            loop {
                let expr = match tokens.peek() {
                    Some(TokenData { value: Token::DoublePeriod, .. }) => {
                        tokens.pop();
                        Expression::ListValueSpread(Box::new(build_expression(tokens)?))
                    },
                    _ => build_expression(tokens)?,
                };

                exprs.push(Box::new(expr));

                match tokens.pop() {
                    Some(TokenData { value: Token::Comma, .. }) => {
                        let new_line = ignore_new_lines(tokens);

                        match tokens.peek() {
                            Some(TokenData { value: Token::CloseBracket, .. }) => {
                                tokens.pop();
                                break;
                            },
                            _ => {}
                        }

                        if let Some(new_line) = new_line {
                            tokens.push(new_line);
                        }
                    },
                    Some(TokenData { value: Token::CloseBracket, .. }) => break,
                    Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
                    None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBracket)))?,
                }

                ignore_new_lines(tokens);
            }

            List::Explicit(ListExplicit { values: exprs })
        },
        Some(TokenData { value: Token::Keyword(Keyword::For), .. }) => {
            let item = build_assignment_target(tokens)?;

            ignore_new_lines(tokens);
            
            match tokens.pop() {
                Some(TokenData { value: Token::Keyword(Keyword::In), .. }) => {},
                Some(token) => Err(ParseError::UnexpectedToken(token))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::In))))?,
            }

            ignore_new_lines(tokens);

            let r#in = Box::new(build_expression(tokens)?);

            ignore_new_lines(tokens);

            match tokens.pop() {
                Some(TokenData { value: Token::CloseBracket, .. }) => {},
                Some(token) => Err(ParseError::UnexpectedToken(token))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBracket)))?,
            }

            List::FnFor(ListFnFor {
                expression: Box::new(first_expr),
                item,
                r#in,
            })
        },
        Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Comma)))?,
    };


    return Ok(Expression::List(list));
}

