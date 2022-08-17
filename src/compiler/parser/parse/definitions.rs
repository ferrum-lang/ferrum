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
        Some(TokenData { value: Token::Keyword(Keyword::Fn), .. }) => Definition::Function(build_definition_fn(tokens, is_public)?),
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

fn build_definition_fn(tokens: &mut Stack<TokenData>, is_public: bool) -> Result<DefFn> {
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

    return Ok(DefFn {
        r#impl,
        signature,
    });
}

fn build_generics(tokens: &mut Stack<TokenData>) -> Result<DefGenerics> {
    todo!();
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

    return Ok(Definition::Type(DefType::Struct(DefStruct {
        name,
        fields,
        generics,
        is_public,
    })));
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

    let construct = match tokens.peek() {
        Some(TokenData { value: Token::Keyword(Keyword::Construct), .. }) => todo!(),
        _ => None,
    };

    let mut functions = vec![];
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

        let is_public = match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Pub), .. }) => {
                tokens.pop();
                true
            }
            _ => false,
        };

        let is_mut = match tokens.peek() {
            Some(TokenData { value: Token::Keyword(Keyword::Mut), .. }) => {
                tokens.pop();
                true
            }
            _ => false,
        };

        match tokens.pop() {
            Some(TokenData { value: Token::Keyword(Keyword::Fn), .. }) if !is_mut => {
                let function = build_definition_fn(tokens, is_public)?;
                functions.push(function);
            },
            Some(TokenData { value: Token::Keyword(Keyword::Self_), .. }) => {
                let method = build_def_class_method(tokens, is_public, is_mut)?;
                methods.push(method);
            },
            Some(token) => Err(ParseError::UnexpectedToken(token))?,
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
        r#type,
        r#default,
        is_public,
    });
}

fn build_def_class_method(tokens: &mut Stack<TokenData>, is_public: bool, is_mut: bool) -> Result<DefClassMethod> {
    todo!();
}

fn build_definition_interface(tokens: &mut Stack<TokenData>, is_public: bool) -> Result<Definition> {
    todo!();
}

fn build_definition_enum(tokens: &mut Stack<TokenData>, is_public: bool) -> Result<Definition> {
    todo!();
}

fn build_definition_errors(tokens: &mut Stack<TokenData>, is_public: bool) -> Result<Definition> {
    todo!();
}

