use super::{symbols::Symbol, syntax::*, Error};

pub fn parse_symbols(mut symbols: Vec<Symbol>) -> Result<SyntaxTree, Error> {
    println!("Building Syntax Tree From:\n{:?}\n", symbols);

    symbols.reverse();

    let mut syntax_tree = SyntaxTree::new();

    let mut is_public = false;

    while let Some(symbol) = symbols.pop() {
        match symbol {
            Symbol::Import => parse_import(&mut symbols, &mut syntax_tree, symbol)?,
            Symbol::Function => parse_function(&mut symbols, &mut syntax_tree, symbol, is_public)?,
            Symbol::Public => {
                is_public = true;
                continue;
            }
            _ => todo!(
                "Unexpected symbol: {:?}\n\nSyntax Tree: {:?}",
                symbol,
                syntax_tree
            ),
        }

        is_public = false;
    }

    return Ok(syntax_tree);
}

fn parse_import(
    symbols: &mut Vec<Symbol>,
    syntax_tree: &mut SyntaxTree,
    _: Symbol,
) -> Result<(), Error> {
    let symbol = symbols.pop().expect("Unfinished import!");

    match symbol {
        Symbol::DestructureOpenBrace => {
            let mut fields = vec![];

            loop {
                let symbol = symbols.pop().expect("Unfinished import!");

                match symbol {
                    Symbol::DestructureField(field) => {
                        let symbol = symbols.last().expect("Unfinished import!");

                        match symbol {
                            Symbol::DestructureAliasColon => {
                                symbols.pop();

                                match symbols.pop().expect("Unfinished import!") {
                                    Symbol::DestructureAliasName(alias) => {
                                        fields.push(DestructureAssignmentFieldNode {
                                            field_token: field,
                                            alias: Some(DestructureAssignmentFieldAliasNode {
                                                name_token: alias,
                                            }),
                                        });
                                    }
                                    symbol => todo!(
                                        "Unexpected symbol: {:?}\n\n{:?}",
                                        symbol,
                                        syntax_tree
                                    ),
                                }
                            }
                            _ => {
                                fields.push(DestructureAssignmentFieldNode {
                                    field_token: field,
                                    alias: None,
                                });
                            }
                        }
                    }
                    Symbol::DestructureComma => {}
                    Symbol::DestructureCloseBrace => {
                        break;
                    }
                    _ => todo!("Unexpected symbol: {:?}", symbol),
                }
            }

            let assignment =
                ImportAssignmentNode::Destructured(DestructureAssignmentNode { fields });

            match symbols.pop().expect("Unfinished import!") {
                Symbol::ImportFrom => match symbols.pop().expect("Unfinished import!") {
                    Symbol::ImportSource(source) => {
                        syntax_tree.imports.push(ImportNode {
                            assignment,
                            source_token: source,
                        });
                    }
                    symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                },
                symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
            }

            match symbols.pop().expect("Unfinished import!") {
                Symbol::Semicolon => {}
                symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
            }
        }
        _ => todo!("Unexpected symbol: {:?}", symbol),
    }

    return Ok(());
}

fn parse_function(
    mut symbols: &mut Vec<Symbol>,
    mut syntax_tree: &mut SyntaxTree,
    _: Symbol,
    is_public: bool,
) -> Result<(), Error> {
    let function_name;

    loop {
        let symbol = symbols.pop().expect("Unfinished function!");

        match symbol {
            Symbol::FunctionName(name) => {
                function_name = name;
                break;
            }
            _ => todo!("Unexpected symbol: {:?}", symbol),
        }
    }

    match symbols.pop().expect("Unfinished function!") {
        Symbol::FunctionParamsOpenParenthesis => {}
        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
    }

    let symbol = symbols.pop().expect("Unfinished function!");

    let mut params = match symbol {
        Symbol::FunctionParamsParamName(_) => vec![build_function_param_node(
            &mut symbols,
            &mut syntax_tree,
            symbol,
        )?],
        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
    };

    loop {
        let symbol = symbols.pop().expect("Unfinished function!");

        match symbol {
            Symbol::FunctionParamsComma => {
                let symbol = symbols.pop().expect("Unfinished function!");

                params.push(build_function_param_node(
                    &mut symbols,
                    &mut syntax_tree,
                    symbol,
                )?);
            }
            Symbol::FunctionParamsCloseParenthesis => {
                break;
            }
            symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
        }
    }

    let signature = FunctionSignatureNode {
        is_public,
        name_token: function_name,
        params,
        return_type: None,
    };

    match symbols.pop().expect("Unfinished function!") {
        Symbol::FunctionExpressionsOpenBrace => {}
        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
    }

    let mut statements = vec![];

    loop {
        let symbol = symbols.pop().expect("Unfinished function!");

        match symbol {
            Symbol::FunctionExpressionsCloseBrace => {
                break;
            }
            symbol => statements.push(build_statement_node(
                &mut symbols,
                &mut syntax_tree,
                symbol,
            )?),
        }
    }

    let body = FunctionBodyNode { statements };

    syntax_tree
        .items
        .push(ItemNode::Function(FunctionNode { signature, body }));

    return Ok(());
}

fn build_function_param_node(
    symbols: &mut Vec<Symbol>,
    syntax_tree: &mut SyntaxTree,
    symbol: Symbol,
) -> Result<FunctionParamNode, Error> {
    match symbol {
        Symbol::FunctionParamsParamName(param_name) => {
            match symbols.pop().expect("Unfinished function param!") {
                Symbol::FunctionParamsParamTypeColon => {}
                symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
            };

            let is_mutable = match symbols.last().expect("Unfinished function param!") {
                Symbol::FunctionParamsParamTypeMutable => {
                    symbols.pop();
                    true
                }
                _ => false,
            };

            let is_borrowed = match symbols.last().expect("Unfinished function param!") {
                Symbol::FunctionParamsParamTypeBorrowed => {
                    symbols.pop();
                    true
                }
                _ => false,
            };

            let type_name = match symbols.pop().expect("Unfinished function param!") {
                Symbol::FunctionParamsParamTypeName(name) => name,
                symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
            };

            return Ok(FunctionParamNode {
                name_token: param_name,
                is_mutable,
                is_borrowed,
                type_token: type_name,
            });
        }
        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
    }
}

fn build_statement_node(
    mut symbols: &mut Vec<Symbol>,
    mut syntax_tree: &mut SyntaxTree,
    symbol: Symbol,
) -> Result<StatementNode, Error> {
    match symbol {
        Symbol::TypeAccessName(_) | Symbol::FunctionCallName(_) => {
            let node = StatementNode::Expression(build_expression_node(
                &mut symbols,
                &mut syntax_tree,
                symbol,
            )?);

            match symbols.pop().expect("Unfinished expression!") {
                Symbol::Semicolon => {}
                symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
            }

            return Ok(node);
        }
        Symbol::Const => match symbols.pop().expect("Unfinished expression!") {
            Symbol::VariableName(variable_name) => {
                let explicit_type = if symbols.last() == Some(&Symbol::VariableTypeColon) {
                    symbols.pop();

                    let symbol = symbols.pop().expect("Unfinished expression!");

                    Some(build_type_node(&mut symbols, &mut syntax_tree, symbol)?)
                } else {
                    None
                };

                match symbols.pop().expect("Unfinished expression!") {
                    Symbol::Assignment => {}
                    symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                }

                let symbol = symbols.pop().expect("Unfinised expression!");

                let node = StatementNode::Assignment(AssignmentNode {
                    left: AssignmentLeftNode {
                        reassignable: false,
                        name_token: variable_name,
                        explicit_type,
                    },
                    right: build_expression_node(&mut symbols, &mut syntax_tree, symbol)?,
                });

                match symbols.pop().expect("Unfinished expression!") {
                    Symbol::Semicolon => {}
                    symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, node),
                }

                return Ok(node);
            }
            symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
        },
        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
    }
}

fn build_type_node(
    mut symbols: &mut Vec<Symbol>,
    mut syntax_tree: &mut SyntaxTree,
    symbol: Symbol,
) -> Result<TypeNode, Error> {
    let mut is_borrowed = false;
    let mut is_mutable = false;

    let mut symbol = symbol;

    loop {
        match symbol {
            Symbol::TypeBorrowed => {
                is_borrowed = true;
            }
            Symbol::TypeMutable => {
                is_mutable = true;
            }
            Symbol::TypeName(name) => match name.as_str() {
                "boolean" | "uint" | "uint1" | "uint8" | "uint16" | "uint32" | "uint64"
                | "uint128" | "uint256" | "biguint" | "bit" | "byte" | "int" | "int8" | "int16"
                | "int32" | "int64" | "int128" | "int256" | "bigint" | "float32" | "float64"
                | "char" | "string" => {
                    assert_ne!(is_mutable, true);

                    let is_optional = match symbols.last() {
                        Some(Symbol::TypeOptional) => {
                            symbols.pop();
                            true
                        }
                        _ => false,
                    };

                    return Ok(TypeNode::Primative(PrimativeTypeNode {
                        name_token: name,
                        is_borrowed,
                        is_optional,
                    }));
                }
                _ => {
                    let is_optional = match symbols.last() {
                        Some(Symbol::TypeOptional) => {
                            symbols.pop();
                            true
                        }
                        _ => false,
                    };

                    let generics = match symbols.last() {
                        Some(Symbol::TypeGenericOpen) => {
                            symbols.pop();

                            let mut symbol = symbols.pop().expect("Unfinished type!");
                            let mut generics =
                                vec![build_type_node(&mut symbols, &mut syntax_tree, symbol)?];

                            loop {
                                symbol = symbols.pop().expect("Unfinished type!");

                                match symbol {
                                    Symbol::TypeGenericComma => {
                                        symbol = symbols.pop().expect("Unfinished type!");
                                        generics.push(build_type_node(
                                            &mut symbols,
                                            &mut syntax_tree,
                                            symbol,
                                        )?);
                                    }
                                    Symbol::TypeGenericClose => {
                                        break;
                                    }
                                    symbol => todo!(
                                        "Unexpected symbol: {:?}\n\n{:?}",
                                        symbol,
                                        syntax_tree
                                    ),
                                }
                            }

                            Some(generics)
                        }
                        _ => None,
                    };

                    return Ok(TypeNode::Structure(StructureTypeNode {
                        path: StructureTypePathNode {
                            segments: vec![StructureTypePathSegmentNode { name_token: name }],
                        },
                        generics,
                        is_borrowed,
                        is_mutable,
                        is_optional,
                    }));
                }
            },
            Symbol::TupleTypeStart => {
                let symbol = symbols.pop().expect("Unfinied tuple type!");

                let tuple_type = build_type_node(&mut symbols, &mut syntax_tree, symbol)?;

                let symbol = symbols.pop().expect("Unfinied tuple type!");

                match symbol {
                    Symbol::TupleTypeSemicolon => {
                        let length = match symbols.pop().expect("Unfinished tuple type!") {
                            Symbol::TupleTypeLength(length) => length,
                            symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                        };

                        match symbols.pop().expect("Unfinished tuple type!") {
                            Symbol::TupleTypeEnd => {}
                            symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                        }

                        let is_optional = match symbols.last() {
                            Some(Symbol::TypeOptional) => {
                                symbols.pop();
                                true
                            }
                            _ => false,
                        };

                        return Ok(TypeNode::Tuple(TupleTypeNode {
                            segments: vec![tuple_type; length],
                            is_optional,
                        }));
                    }
                    _ => {
                        symbols.push(symbol);

                        let mut segments = vec![tuple_type];

                        loop {
                            let symbol = symbols.pop().expect("Unfinished tuple type!");

                            match symbol {
                                Symbol::TupleTypeComma => {
                                    let symbol = symbols.pop().expect("Unfinished tuple type!");

                                    let tuple_type =
                                        build_type_node(&mut symbols, &mut syntax_tree, symbol)?;
                                    segments.push(tuple_type);
                                }
                                Symbol::TupleTypeEnd => {
                                    let is_optional = match symbols.last() {
                                        Some(Symbol::TypeOptional) => {
                                            symbols.pop();
                                            true
                                        }
                                        _ => false,
                                    };

                                    return Ok(TypeNode::Tuple(TupleTypeNode {
                                        segments,
                                        is_optional,
                                    }));
                                }
                                symbol => {
                                    todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree)
                                }
                            }
                        }
                    }
                }
            }
            symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
        }

        symbol = symbols.pop().expect("Unfinished type!");
    }
}

fn build_expression_node(
    mut symbols: &mut Vec<Symbol>,
    mut syntax_tree: &mut SyntaxTree,
    symbol: Symbol,
) -> Result<ExpressionNode, Error> {
    let mut expression = match symbol {
        Symbol::True => ExpressionNode::Literal(LiteralDataNode::Boolean(true)),
        Symbol::False => ExpressionNode::Literal(LiteralDataNode::Boolean(false)),
        Symbol::Int(value) => ExpressionNode::Literal(LiteralDataNode::Integer(value)),
        Symbol::Char(value) => ExpressionNode::Literal(LiteralDataNode::Char(value)),
        Symbol::PlainString(value) => ExpressionNode::Literal(LiteralDataNode::PlainString(value)),
        Symbol::ListOpen => {
            let symbol = symbols.pop().expect("Unfinished expression!");
            let value = build_expression_node(&mut symbols, &mut syntax_tree, symbol)?;

            let symbol = symbols.pop().expect("Unfinished expression!");

            match symbol {
                Symbol::ListFor => {
                    let for_name_token = match symbols.pop().expect("Unfinished list!") {
                        Symbol::VariableName(name) => name,
                        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                    };

                    match symbols.pop().expect("Unfinished list!") {
                        Symbol::ListForIn => {}
                        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                    }

                    let symbol = symbols.pop().expect("Unfinished list!");
                    let range = build_expression_node(&mut symbols, &mut syntax_tree, symbol)?;

                    match symbols.pop().expect("Unfinished list!") {
                        Symbol::ListClose => {}
                        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                    }

                    ExpressionNode::Literal(LiteralDataNode::List(ListNode::ForIn(ForInListNode {
                        expression: Box::new(value),
                        for_name_token,
                        range: Box::new(range),
                    })))
                }
                _ => {
                    symbols.push(symbol);

                    let mut segments = vec![value];

                    let expression;

                    loop {
                        let symbol = symbols.pop().expect("Unfinished expression!");

                        match symbol {
                            Symbol::ListComma => {
                                let symbol = symbols.pop().expect("Unfinished expression!");
                                let value =
                                    build_expression_node(&mut symbols, &mut syntax_tree, symbol)?;

                                segments.push(value);
                            }
                            Symbol::ListClose => {
                                expression = ExpressionNode::Literal(LiteralDataNode::List(
                                    ListNode::Segmented(SegmentedListNode { segments }),
                                ));
                                break;
                            }
                            symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                        }
                    }

                    expression
                }
            }
        }
        Symbol::TupleStart => {
            let symbol = symbols.pop().expect("Unfinished expression!");
            let value = build_expression_node(&mut symbols, &mut syntax_tree, symbol)?;

            let symbol = symbols.pop().expect("Unfinished expression!");

            match symbol {
                Symbol::TupleSemicolon => {
                    let length = match symbols.pop().expect("Unfinished tuple!") {
                        Symbol::TupleLength(length) => length,
                        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                    };

                    match symbols.pop().expect("Unfinished tuple!") {
                        Symbol::TupleEnd => {}
                        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                    }

                    ExpressionNode::Literal(LiteralDataNode::Tuple(TupleNode {
                        segments: vec![value; length],
                    }))
                }
                _ => {
                    symbols.push(symbol);

                    let mut segments = vec![value];

                    let expression;

                    loop {
                        let symbol = symbols.pop().expect("Unfinished expression!");

                        match symbol {
                            Symbol::TupleComma => {
                                let symbol = symbols.pop().expect("Unfinished expression!");
                                let value =
                                    build_expression_node(&mut symbols, &mut syntax_tree, symbol)?;

                                segments.push(value);
                            }
                            Symbol::TupleEnd => {
                                expression =
                                    ExpressionNode::Literal(LiteralDataNode::Tuple(TupleNode {
                                        segments,
                                    }));
                                break;
                            }
                            symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                        }
                    }

                    expression
                }
            }
        }
        Symbol::TemplateStringStart(start) => {
            let expression;
            let mut middle_tokens = vec![];
            let mut expressions = vec![];

            loop {
                let symbol = symbols.pop().expect("Unfinished expression!");

                match symbol {
                    Symbol::TemplateStringMiddle(middle) => {
                        middle_tokens.push(middle);
                    }
                    Symbol::TemplateStringEnd(end) => {
                        expression = ExpressionNode::Literal(LiteralDataNode::TemplateString(
                            TemplateStringNode {
                                start_token: start,
                                middle_tokens,
                                expressions,
                                end_token: end,
                            },
                        ));
                        break;
                    }
                    Symbol::TemplateStringTemplateOpenBrace => {
                        let symbol = symbols.pop().expect("Unfinished expression!");
                        let expression =
                            build_expression_node(&mut symbols, &mut syntax_tree, symbol)?;

                        expressions.push(expression);

                        match symbols.pop().expect("Unfinished expression!") {
                            Symbol::TemplateStringTemplateCloseBrace => {}
                            symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                        }
                    }
                    symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                }
            }

            expression
        }
        Symbol::Mutable => {
            let is_borrowed = match symbols.last() {
                Some(Symbol::InstanceBorrow) => true,
                _ => false,
            };

            match symbols.pop().expect("Unfinished expression!") {
                Symbol::InstanceReferenceName(instance_reference_name) => {
                    ExpressionNode::InstanceReference(InstanceReferenceNode {
                        name_token: instance_reference_name,
                        is_mutable: true,
                        is_borrowed,
                    })
                }
                symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
            }
        }
        Symbol::InstanceBorrow => match symbols.pop().expect("Unfinished expression!") {
            Symbol::InstanceReferenceName(instance_reference_name) => {
                ExpressionNode::InstanceReference(InstanceReferenceNode {
                    name_token: instance_reference_name,
                    is_borrowed: true,
                    is_mutable: false,
                })
            }
            symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
        },
        Symbol::InstanceReferenceName(instance_reference_name) => {
            ExpressionNode::InstanceReference(InstanceReferenceNode {
                name_token: instance_reference_name,
                is_borrowed: false,
                is_mutable: false,
            })
        }
        Symbol::InstanceAccessName(instance_access_name) => {
            match symbols.pop().expect("Unfinished expression!") {
                Symbol::InstanceAccessPeriod => {}
                symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
            }

            let symbol = symbols.pop().expect("Unfinished expression!");

            let right = match build_expression_node(&mut symbols, &mut syntax_tree, symbol)? {
                ExpressionNode::Call(node) => InstanceAccessRightNode::Call(node),
                ExpressionNode::InstanceReference(node) => InstanceAccessRightNode::Reference(node),
                ExpressionNode::InstanceAccess(InstanceAccessNode { right, left: _ }) => {
                    InstanceAccessRightNode::Access(Box::new(right))
                }
                ExpressionNode::Literal(LiteralDataNode::Integer(value)) => {
                    InstanceAccessRightNode::Reference(InstanceReferenceNode {
                        name_token: String::from(value),
                        is_borrowed: false,
                        is_mutable: false,
                    })
                }
                expression => todo!(
                    "Unexpected expression: {:?}\n\n{:?}",
                    expression,
                    syntax_tree
                ),
            };

            ExpressionNode::InstanceAccess(InstanceAccessNode {
                left: Box::new(ExpressionNode::InstanceReference(InstanceReferenceNode {
                    name_token: instance_access_name,
                    is_mutable: false,
                    is_borrowed: false,
                })),
                right,
            })
        }
        Symbol::TypeAccessName(_) => {
            build_call_node_expression(&mut symbols, &mut syntax_tree, symbol, vec![])?
        }
        Symbol::FunctionCallName(_) => {
            build_call_node_expression(&mut symbols, &mut syntax_tree, symbol, vec![])?
        }
        Symbol::ClosureParamsOpen => {
            build_closure_expression(&mut symbols, &mut syntax_tree, symbol)?
        }
        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
    };

    loop {
        match symbols.last().expect("Unfinished!") {
            Symbol::Plus => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::Add,
                });
            }
            Symbol::Minus => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::Subtract,
                });
            }
            Symbol::Multiply => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::Multiply,
                });
            }
            Symbol::Divide => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::Divide,
                });
            }
            Symbol::Exponent => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::Exponent,
                });
            }
            Symbol::Modulo => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::Modulo,
                });
            }
            Symbol::Or => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::Or,
                });
            }
            Symbol::And => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::And,
                });
            }
            Symbol::Eq => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::Eq,
                });
            }
            Symbol::NotEq => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::NotEq,
                });
            }
            Symbol::Gt => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::Gt,
                });
            }
            Symbol::GtOrEq => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::GtOrEq,
                });
            }
            Symbol::Lt => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::Lt,
                });
            }
            Symbol::LtOrEq => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::Binary(BinaryExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                    op: BinaryOpNode::LtOrEq,
                });
            }
            Symbol::NullCoalesce => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");

                expression = ExpressionNode::NullCoalesce(NullCoalesceExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                });
            }
            Symbol::CastQuestion => {
                symbols.pop();

                expression = ExpressionNode::QuestionCast(QuestionCastNode {
                    expression: Box::new(expression),
                });
            }
            Symbol::Range => {
                symbols.pop();

                let symbol = symbols.pop().expect("Unfinished!");
                let right = build_expression_node(&mut symbols, &mut syntax_tree, symbol)?;

                return Ok(ExpressionNode::Range(RangeExpressionNode {
                    left: Box::new(expression),
                    right: Box::new(right),
                }));
            }
            Symbol::InstanceAccessPeriod => {
                todo!();
            }
            _ => return Ok(expression),
        }
    }
}

fn build_call_node_expression(
    mut symbols: &mut Vec<Symbol>,
    mut syntax_tree: &mut SyntaxTree,
    symbol: Symbol,
    mut segments: Vec<ExpressionCallPathSegmentNode>,
) -> Result<ExpressionNode, Error> {
    let expression;

    match symbol {
        Symbol::TypeAccessName(type_access_name) => {
            segments.push(ExpressionCallPathSegmentNode::TypeIdentity(
                type_access_name,
            ));

            match symbols.pop().expect("Unfinished call!") {
                Symbol::TypeAccessDoubleSemicolon => {}
                symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
            }

            loop {
                let symbol = symbols.pop().expect("Unfinished call!");

                match symbol {
                    Symbol::FunctionCallName(_) => {
                        expression = build_call_node_expression(
                            &mut symbols,
                            &mut syntax_tree,
                            symbol,
                            segments,
                        )?;

                        break;
                    }
                    symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
                }
            }
        }
        Symbol::FunctionCallName(function_call_name) => {
            segments.push(ExpressionCallPathSegmentNode::FunctionIdentity(
                function_call_name,
            ));

            let call_path = ExpressionCallPathNode { segments };

            let mut args = vec![];

            match symbols.pop().expect("Unfinished function!") {
                Symbol::FunctionCallOpenParenthesis => {}
                symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
            }

            loop {
                let symbol = symbols.pop().expect("Unfinished function!");

                match symbol {
                    Symbol::FunctionCallCloseParenthesis => {
                        break;
                    }
                    symbol => args.push(build_expression_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?),
                }
            }

            expression = ExpressionNode::Call(ExpressionCallNode {
                subject: call_path,
                args,
            });
        }
        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
    }

    return match symbols.last().expect("Unfinished call!") {
        Symbol::InstanceAccessPeriod => {
            symbols.pop();

            let symbol = symbols.pop().expect("Unfinished call!");

            let right = match build_expression_node(&mut symbols, &mut syntax_tree, symbol)? {
                ExpressionNode::Call(node) => InstanceAccessRightNode::Call(node),
                ExpressionNode::InstanceReference(node) => InstanceAccessRightNode::Reference(node),
                ExpressionNode::InstanceAccess(InstanceAccessNode { right, left: _ }) => {
                    InstanceAccessRightNode::Access(Box::new(right))
                }
                ExpressionNode::Literal(LiteralDataNode::Integer(value)) => {
                    InstanceAccessRightNode::Reference(InstanceReferenceNode {
                        name_token: String::from(value),
                        is_borrowed: false,
                        is_mutable: false,
                    })
                }
                expression => todo!(
                    "Unexpected expression: {:?}\n\n{:?}",
                    expression,
                    syntax_tree
                ),
            };

            Ok(ExpressionNode::InstanceAccess(InstanceAccessNode {
                left: Box::new(expression),
                right,
            }))
        }
        _ => Ok(expression),
    };
}

fn build_closure_expression(
    mut symbols: &mut Vec<Symbol>,
    mut syntax_tree: &mut SyntaxTree,
    symbol: Symbol,
) -> Result<ExpressionNode, Error> {
    match symbol {
        Symbol::ClosureParamsOpen => {}
        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
    }

    let symbol = symbols.pop().expect("Unfinished closure!");

    let mut params = match symbol {
        Symbol::FunctionParamsParamName(_) => vec![build_closure_param_node(
            &mut symbols,
            &mut syntax_tree,
            symbol,
        )?],
        Symbol::ClosureParamsClose => vec![],
        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
    };

    if params.len() > 0 {
        loop {
            let symbol = symbols.pop().expect("Unfinished closure!");

            match symbol {
                Symbol::ClosureParamsComma => {
                    let symbol = symbols.pop().expect("Unfinished closure!");

                    params.push(build_closure_param_node(
                        &mut symbols,
                        &mut syntax_tree,
                        symbol,
                    )?);
                }
                Symbol::ClosureParamsClose => {
                    break;
                }
                symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
            }
        }
    }

    let signature = ClosureSignatureNode {
        params,
        return_type: None,
    };

    match symbols.pop().expect("Unfinished closure!") {
        Symbol::ClosureArrow => {}
        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
    }

    let mut statements = match symbols.last().expect("Unfinished closure!") {
        Symbol::FunctionExpressionsOpenBrace => {
            symbols.pop();
            vec![]
        }
        _ => {
            let symbol = symbols.pop().expect("Unfinished closure!");
            vec![StatementNode::Expression(build_expression_node(
                &mut symbols,
                &mut syntax_tree,
                symbol,
            )?)]
        }
    };

    if statements.len() == 0 {
        loop {
            let symbol = symbols.pop().expect("Unfinished function!");

            match symbol {
                Symbol::FunctionExpressionsCloseBrace => {
                    break;
                }
                symbol => statements.push(build_statement_node(
                    &mut symbols,
                    &mut syntax_tree,
                    symbol,
                )?),
            }
        }
    }

    let body = FunctionBodyNode { statements };

    return Ok(ExpressionNode::Closure(ClosureExpressionNode {
        signature,
        body,
    }));
}

fn build_closure_param_node(
    symbols: &mut Vec<Symbol>,
    syntax_tree: &mut SyntaxTree,
    symbol: Symbol,
) -> Result<ClosureParamNode, Error> {
    match symbol {
        Symbol::FunctionParamsParamName(param_name) => {
            match symbols.pop().expect("Unfinished function param!") {
                Symbol::FunctionParamsParamTypeColon => {}
                symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
            };

            let is_mutable = match symbols.last().expect("Unfinished function param!") {
                Symbol::FunctionParamsParamTypeMutable => {
                    symbols.pop();
                    true
                }
                _ => false,
            };

            let is_borrowed = match symbols.last().expect("Unfinished function param!") {
                Symbol::FunctionParamsParamTypeBorrowed => {
                    symbols.pop();
                    true
                }
                _ => false,
            };

            let type_name = match symbols.last().expect("Unfinished function param!") {
                Symbol::FunctionParamsParamTypeName(name) => {
                    let name = name.clone();
                    symbols.pop();
                    Some(name)
                }
                _ => None,
            };

            return Ok(ClosureParamNode {
                name_token: param_name,
                is_mutable,
                is_borrowed,
                type_token: type_name,
            });
        }
        symbol => todo!("Unexpected symbol: {:?}\n\n{:?}", symbol, syntax_tree),
    }
}
