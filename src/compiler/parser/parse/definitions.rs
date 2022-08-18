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
        Some(TokenData { value: Token::Keyword(Keyword::Async), .. }) => {
            tokens.pop();

            let is_async = match tokens.peek() {
                Some(TokenData { value: Token::QuestionMark, .. }) => {
                    tokens.pop();
                    MaybeBool::Maybe
                },
                _ => MaybeBool::True,
            };

            Definition::Function(build_definition_fn(tokens, is_public, is_async)?)
        },
        Some(TokenData { value: Token::Keyword(Keyword::Fn), .. }) => {
            const IS_ASYNC: MaybeBool = MaybeBool::False;
            Definition::Function(build_definition_fn(tokens, is_public, IS_ASYNC)?)
        },
        Some(TokenData { value: Token::Keyword(Keyword::Struct), .. }) => build_definition_struct(tokens, is_public)?,
        Some(TokenData { value: Token::Keyword(Keyword::Class), .. }) => build_definition_class(tokens, is_public)?,
        Some(TokenData { value: Token::Keyword(Keyword::Interface), .. }) => build_definition_interface(tokens, is_public)?,
        Some(TokenData { value: Token::Keyword(Keyword::Enum), .. }) => build_definition_enum(tokens, is_public)?,
        Some(TokenData { value: Token::Keyword(Keyword::Errors), .. }) => build_definition_errors(tokens, is_public)?,
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

fn build_definition_fn(tokens: &mut Stack<TokenData>, is_public: bool, is_async: MaybeBool) -> Result<DefFn> {
    let signature = build_def_fn_signature(tokens, is_public, is_async)?;

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

    return Ok(DefFn {
        r#impl,
        signature,
    });
}

fn build_generics(tokens: &mut Stack<TokenData>) -> Result<DefGenerics> {
    match tokens.pop() {
        Some(TokenData { value: Token::LessThan, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::LessThan)))?,
    }

    let mut generics = vec![];

    loop {
        ignore_new_lines(tokens);

        match tokens.peek() {
            Some(TokenData { value: Token::GreaterThan, .. }) => {
                tokens.pop();
                break;
            },
            _ => {},
        }

        let name = match tokens.pop() {
            Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
            Some(token) => Err(ParseError::UnexpectedToken(token))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
        };


        ignore_new_lines(tokens);

        let constraints = match tokens.peek() {
            Some(TokenData { value: Token::Colon, .. }) => {
                tokens.pop();

                let mut types = vec![];

                loop {
                    ignore_new_lines(tokens);

                    let r#type = build_type(tokens)?;

                    types.push(r#type);

                    ignore_new_lines(tokens);

                    match tokens.peek() {
                        Some(TokenData { value: Token::Plus, .. }) => {
                            tokens.pop();
                        },
                        _ => break,
                    }
                }

                Some(DefGenericConstraints { types })
            },
            _ => None,
        };

        generics.push(DefGeneric {
            name,
            constraints,
        });

        match tokens.pop() {
            Some(TokenData { value: Token::GreaterThan, .. }) => break,
            Some(TokenData { value: Token::Comma, .. }) => {},
            Some(token) => Err(ParseError::UnexpectedToken(token))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::GreaterThan)))?,
        }
    }

    return Ok(DefGenerics { generics });
}

fn build_def_fn_signature(tokens: &mut Stack<TokenData>, is_public: bool, is_async: MaybeBool) -> Result<DefFnSignature> {
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
        Some(TokenData { value: Token::LessThan, .. }) => Some(build_generics(tokens)?),
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
        is_async,
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

fn build_definition_struct(tokens: &mut Stack<TokenData>, is_public: bool) -> Result<Definition> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Struct), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Struct))))?,
    }

    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(name), .. }) => name,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    let generics = match tokens.peek() {
        Some(TokenData { value: Token::LessThan, .. }) => Some(build_generics(tokens)?),
        _ => None,
    };

    ignore_new_lines(tokens);

    match tokens.peek() {
        Some(TokenData { value: Token::OpenParenthesis, .. }) => {
            let tuple_type = build_tuple_type(tokens)?;

            return Ok(Definition::Type(DefType::TupleStruct(DefTupleStruct {
                name,
                generics,
                tuple_type,
                is_public,
            })));
        },
        _ => {},
    }

    let fields = build_def_struct_body(tokens)?;

    return Ok(Definition::Type(DefType::Struct(DefStruct {
        name,
        fields,
        generics,
        is_public,
    })));
}

fn build_def_struct_body(tokens: &mut Stack<TokenData>) -> Result<Vec<DefStructField>> {
    match tokens.pop() {
        Some(TokenData { value: Token::OpenBrace, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenBrace)))?,
    }

    let mut fields = vec![];

    loop {
        ignore_new_lines(tokens);

        match tokens.peek() {
            Some(TokenData { value: Token::CloseBrace, .. }) => {
                tokens.pop();
                break;
            },
            _ => {},
        }

        let field = build_def_struct_field(tokens)?;
        fields.push(field);

        match tokens.pop() {
            Some(TokenData { value: Token::CloseBrace, .. }) => break,
            Some(TokenData { value: Token::Comma, .. }) => {},
            Some(token) => Err(ParseError::UnexpectedToken(token))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBrace)))?,
        }
    }

    return Ok(fields);
}

fn build_def_struct_field(tokens: &mut Stack<TokenData>) -> Result<DefStructField> {
    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    match tokens.pop() {
        Some(TokenData { value: Token::Colon, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Colon)))?,
    }

    ignore_new_lines(tokens);

    let r#type = build_type(tokens)?;

    ignore_new_lines(tokens);

    let r#default = match tokens.peek() {
        Some(TokenData { value: Token::Equals, .. }) => {
            tokens.pop();

            Some(build_expression(tokens)?)
        },
        _ => None,
    };

    return Ok(DefStructField {
        name,
        r#type,
        r#default,
    });
}

fn build_definition_class(tokens: &mut Stack<TokenData>, is_public: bool) -> Result<Definition> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Class), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Class))))?,
    }

    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(name), .. }) => name,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    let generics = match tokens.peek() {
        Some(TokenData { value: Token::LessThan, .. }) => Some(build_generics(tokens)?),
        _ => None,
    };

    ignore_new_lines(tokens);

    match tokens.pop() {
        Some(TokenData { value: Token::OpenBrace, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenBrace)))?,
    }

    ignore_new_lines(tokens);

    let mut static_consts = vec![];

    loop {
        let is_public = match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Pub), .. }) => {
                tokens.pop();
                true
            },
            _ => false,
        };

        match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Static), .. }) => {
                let static_const = build_static_const(tokens, is_public)?;
                static_consts.push(static_const);
            },
            _ => break,
        }

        ignore_new_lines(tokens);
    }

    ignore_new_lines(tokens);

    let self_state = match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::Self_), .. }) => {
            tokens.pop();

            ignore_new_lines(tokens);

            match tokens.pop() {
                Some(TokenData { value: Token::OpenBrace, .. }) => {},
                Some(token) => Err(ParseError::UnexpectedToken(token))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::OpenBrace)))?,
            }

            let mut fields = vec![];

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

                let field = build_def_class_field(tokens)?;
                fields.push(field);

                match tokens.pop() {
                    Some(TokenData { value: Token::CloseBrace, .. }) => break,
                    Some(TokenData { value: Token::Comma, .. }) => {},
                    Some(token) => Err(ParseError::UnexpectedToken(token))?,
                    None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBrace)))?,
                }
            }

            Some(DefClassSelfState { fields })
        },
        _ => None,
    };

    ignore_new_lines(tokens);

    let pub_token = match tokens.peek() {
        Some(token) if token.value == Token::Keyword(Keyword::Pub) => {
            let token = token.clone();
            tokens.pop();
            Some(token)
        }
        _ => None,
    };

    let construct = match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::Construct), .. }) => {
            tokens.pop();

            match tokens.pop() {
                Some(TokenData { value: Token::DoublePeriod, .. }) => Some(DefClassConstruct {
                    is_public: pub_token.is_some(),
                    details: DefClassConstructDetails::Default,
                }),
                Some(TokenData { value: Token::OpenParenthesis, .. }) => {
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

                        let name = match tokens.pop() {
                            Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
                            Some(token) => Err(ParseError::UnexpectedToken(token))?,
                            None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
                        };

                        ignore_new_lines(tokens);

                        let alias = match tokens.peek() {
                            Some(TokenData { value: Token::Colon, .. }) => {
                                tokens.pop();

                                ignore_new_lines(tokens);

                                match tokens.pop() {
                                    Some(TokenData { value: Token::Identifier(ident), .. }) => Some(ident),
                                    Some(token) => Err(ParseError::UnexpectedToken(token))?,
                                    None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
                                }
                            },
                            _ => None,
                        };

                        params.push(DefClassConstructParam::Direct(DefClassConstructParamDirect {
                            name,
                            alias,
                        }));

                        ignore_new_lines(tokens);

                        match tokens.pop() {
                            Some(TokenData { value: Token::CloseParenthesis, .. }) => break,
                            Some(TokenData { value: Token::Comma, .. }) => {},
                            Some(token) => Err(ParseError::UnexpectedToken(token))?,
                            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseParenthesis)))?,
                        }
                    }

                    match tokens.peek() {
                        Some(TokenData { value: Token::DoublePeriod, .. }) => {
                            tokens.pop();

                            Some(DefClassConstruct {
                                is_public,
                                details: DefClassConstructDetails::WithParamsOnly(DefClassConstructWithParams {
                                    params,
                                }),
                            })
                        },
                        _ => {
                            ignore_new_lines(tokens);

                            let body = build_statement_block(tokens)?;

                            Some(DefClassConstruct {
                                is_public,
                                details: DefClassConstructDetails::Full(DefClassConstructFull {
                                    params,
                                    body,
                                })
                            })
                        },
                    }
                }
                Some(token) => Err(ParseError::UnexpectedToken(token))?,
                None => Err(ParseError::MissingExpectedToken(Some(Token::OpenParenthesis)))?,
            }
        },
        _ => {
            if let Some(pub_token) = pub_token {
                tokens.push(pub_token);
            }

            None
        },
    };

    let mut functions = vec![];
    let mut methods = vec![];
    let mut impls = vec![];

    loop {
        ignore_new_lines(tokens);

        match tokens.peek() {
            Some(TokenData { value: Token::CloseBrace, .. }) => {
                tokens.pop();
                break;
            }
            _ => {}
        }

        let is_public = match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Pub), .. }) => {
                tokens.pop();
                true
            }
            _ => false,
        };

        let is_async = match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Async), .. }) => {
                tokens.pop();
                
                match tokens.peek() {
                    Some(TokenData { value: Token::QuestionMark, .. }) => {
                        tokens.pop();
                        MaybeBool::Maybe
                    },
                    _ => MaybeBool::True,
                }
            },
            _ => MaybeBool::False,
        };

        let is_mut = match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Mut), .. }) => {
                tokens.pop();
                true
            }
            _ => false,
        };

        match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Fn), .. }) if !is_mut => {
                let function = build_definition_fn(tokens, is_public, is_async)?;
                functions.push(function);
            },
            Some(TokenData { value: Token::Keyword(Keyword::Self_), .. }) => {
                let method = build_def_class_method(tokens, is_public, is_async, is_mut)?;
                methods.push(method);
            },
            Some(TokenData { value: Token::Keyword(Keyword::Impl), .. }) if !is_public && is_async == MaybeBool::False && !is_mut => {
                let r#impl = build_def_class_impl(tokens)?;
                impls.push(r#impl);
            },
            Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Self_))))?,
        }

        match tokens.pop() {
            Some(TokenData { value: Token::CloseBrace, .. }) => break,
            Some(TokenData { value: Token::NewLine, .. }) => {},
            Some(token) => Err(ParseError::UnexpectedToken(token))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBrace)))?,
        }
    }

    Ok(Definition::Type(DefType::Class(DefClass {
        is_public,
        name,
        static_consts,
        generics,
        functions,
        self_state,
        construct,
        methods,
        impls,
    })))
}

fn build_def_class_field(tokens: &mut Stack<TokenData>) -> Result<DefClassSelfStateField> {
    let is_public = match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::Pub), .. }) => {
            tokens.pop();
            true
        },
        _ => false,
    };

    let is_const = match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Const), .. }) => true,
        Some(TokenData { value: Token::Keyword(Keyword::Let), .. }) => false,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Const))))?,
    };

    ignore_new_lines(tokens);

    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(name), .. }) => name,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    ignore_new_lines(tokens);

    match tokens.pop() {
        Some(TokenData { value: Token::Colon, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Colon)))?,
    }

    ignore_new_lines(tokens);

    let is_mut = match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::Mut), .. }) => {
            tokens.pop();
            true
        },
        _ => false,
    };

    let r#type = build_type(tokens)?;

    ignore_new_lines(tokens);

    let r#default = match tokens.peek() {
        Some(TokenData { value: Token::Equals, .. }) => {
            tokens.pop();

            ignore_new_lines(tokens);

            Some(build_expression(tokens)?)
        },
        _ => None,
    };

    return Ok(DefClassSelfStateField {
        name,
        is_const,
        is_mut,
        r#type,
        r#default,
        is_public,
    });
}

fn build_def_class_method(tokens: &mut Stack<TokenData>, is_public: bool, is_async: MaybeBool, is_mut: bool) -> Result<DefClassMethod> {
    let signature = build_def_class_method_signature(tokens, is_public, is_async, is_mut)?;

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

    return Ok(DefClassMethod {
        signature,
        r#impl,
    });
}

fn build_def_class_method_signature(tokens: &mut Stack<TokenData>, is_public: bool, is_async: MaybeBool, is_mut: bool) -> Result<DefClassMethodSignature> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Self_), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Self_))))?,
    }

    match tokens.pop() {
        Some(TokenData { value: Token::Period, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Period)))?,
    }

    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    let generics = match tokens.peek() {
        Some(TokenData { value: Token::LessThan, .. }) => Some(build_generics(tokens)?),
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

    return Ok(DefClassMethodSignature {
        is_public,
        is_async,
        is_mut,
        name,
        generics,
        params,
        return_type,
    });
}

fn build_def_class_impl(tokens: &mut Stack<TokenData>) -> Result<DefClassImpl> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Impl), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Impl))))?,
    }

    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    ignore_new_lines(tokens);

    match tokens.pop() {
        Some(TokenData { value: Token::OpenBrace, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenBrace)))?,
    }

    let mut methods = vec![];

    loop {
        ignore_new_lines(tokens);

        match tokens.peek() {
            Some(TokenData { value: Token::CloseBrace, .. }) => {
                tokens.pop();
                break;
            }
            _ => {}
        }

        let is_async = match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Async), .. }) => {
                tokens.pop();
                
                match tokens.peek() {
                    Some(TokenData { value: Token::QuestionMark, .. }) => {
                        tokens.pop();

                        MaybeBool::Maybe
                    },
                    _ => MaybeBool::True,
                }
            }
            _ => MaybeBool::False,
        };

        let is_mut = match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Mut), .. }) => {
                tokens.pop();
                true
            }
            _ => false,
        };

        match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Self_), .. }) => {
                const IS_PUBLIC: bool = true;

                let method = build_def_class_method(tokens, IS_PUBLIC, is_async, is_mut)?;
                methods.push(method);
            },
            Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Self_))))?,
        }

        match tokens.pop() {
            Some(TokenData { value: Token::CloseBrace, .. }) => break,
            Some(TokenData { value: Token::NewLine, .. }) => {},
            Some(token) => Err(ParseError::UnexpectedToken(token))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBrace)))?,
        }
    }

    return Ok(DefClassImpl {
        name,
        methods,
    });
}

fn build_definition_interface(tokens: &mut Stack<TokenData>, is_public: bool) -> Result<Definition> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Interface), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Interface))))?,
    }

    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(name), .. }) => name,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    let generics = match tokens.peek() {
        Some(TokenData { value: Token::LessThan, .. }) => Some(build_generics(tokens)?),
        _ => None,
    };

    ignore_new_lines(tokens);

    match tokens.pop() {
        Some(TokenData { value: Token::OpenBrace, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenBrace)))?,
    }

    ignore_new_lines(tokens);

    let mut methods = vec![];

    loop {
        ignore_new_lines(tokens);

        match tokens.peek() {
            Some(TokenData { value: Token::CloseBrace, .. }) => {
                tokens.pop();
                break
            }
            _ => {}
        }

        let is_async = match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Async), .. }) => {
                tokens.pop();
                
                match tokens.peek() {
                    Some(TokenData { value: Token::QuestionMark, .. }) => {
                        tokens.pop();

                        MaybeBool::Maybe
                    },
                    _ => MaybeBool::True,
                }
            }
            _ => MaybeBool::False,
        };

        let is_mut = match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Mut), .. }) => {
                tokens.pop();
                true
            }
            _ => false,
        };

        match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Self_), .. }) => {
                let method = build_def_interface_method(tokens, is_async, is_mut)?;
                methods.push(method);
            },
            Some(token) => Err(ParseError::UnexpectedToken(token.clone()))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Self_))))?,
        }

        match tokens.pop() {
            Some(TokenData { value: Token::CloseBrace, .. }) => break,
            Some(TokenData { value: Token::NewLine, .. }) => {},
            Some(token) => Err(ParseError::UnexpectedToken(token))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBrace)))?,
        }
    }

    return Ok(Definition::Type(DefType::Interface(DefInterface {
        is_public,
        name,
        generics,
        methods,
    })));
}

fn build_def_interface_method(tokens: &mut Stack<TokenData>, is_async: MaybeBool, is_mut: bool) -> Result<DefInterfaceMethod> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Self_), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Self_))))?,
    }

    match tokens.pop() {
        Some(TokenData { value: Token::Period, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Period)))?,
    }

    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    let generics = match tokens.peek() {
        Some(TokenData { value: Token::LessThan, .. }) => Some(build_generics(tokens)?),
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

    return Ok(DefInterfaceMethod {
        is_async,
        is_mut,
        name,
        generics,
        params,
        return_type,
    });
}

fn build_definition_enum(tokens: &mut Stack<TokenData>, is_public: bool) -> Result<Definition> {
    match tokens.pop() {
        Some(TokenData { value: Token::Keyword(Keyword::Enum), .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Keyword(Keyword::Enum))))?,
    }

    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    let generics = match tokens.peek() {
        Some(TokenData { value: Token::LessThan, .. }) => Some(build_generics(tokens)?),
        _ => None,
    };

    ignore_new_lines(tokens);

    match tokens.pop() {
        Some(TokenData { value: Token::OpenBrace, .. }) => {},
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::OpenBrace)))?,
    }

    let mut values = vec![];

    loop {
        ignore_new_lines(tokens);

        match tokens.peek() {
            Some(TokenData { value: Token::CloseBrace, .. }) => {
                tokens.pop();
                break;
            },
            _ => {},
        }

        let field = build_def_enum_field(tokens)?;
        values.push(field);

        ignore_new_lines(tokens);

        match tokens.pop() {
            Some(TokenData { value: Token::CloseBrace, .. }) => break,
            Some(TokenData { value: Token::Comma, .. }) => {},
            Some(token) => Err(ParseError::UnexpectedToken(token))?,
            None => Err(ParseError::MissingExpectedToken(Some(Token::CloseBrace)))?,
        }
    }

    return Ok(Definition::Type(DefType::Enum(DefEnum {
        is_public,
        name,
        generics,
        values,
    })));
}

fn build_def_enum_field(tokens: &mut Stack<TokenData>) -> Result<DefEnumField> {
    let name = match tokens.pop() {
        Some(TokenData { value: Token::Identifier(ident), .. }) => ident,
        Some(token) => Err(ParseError::UnexpectedToken(token))?,
        None => Err(ParseError::MissingExpectedToken(Some(Token::Identifier(String::new()))))?,
    };

    let data = match tokens.peek() {
        Some(TokenData { value: Token::OpenParenthesis, .. }) => {
            Some(DefEnumFieldData::Tuple(build_tuple_type(tokens)?))
        },
        Some(TokenData { value: Token::OpenBrace, .. }) => {
            Some(DefEnumFieldData::Struct(build_def_struct_body(tokens)?))
        },
        _ => None,
    };

    ignore_new_lines(tokens);

    let const_value = match tokens.peek() {
        Some(TokenData { value: Token::Equals, .. }) => todo!(),
        _ => None,
    };

    return Ok(DefEnumField {
        name,
        data,
        const_value,
    });
}

fn build_definition_errors(tokens: &mut Stack<TokenData>, is_public: bool) -> Result<Definition> {
    todo!();
}

