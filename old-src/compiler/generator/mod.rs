use super::{syntax::*, Error};

const IMPORT_PREFIX: &'static str = "fe_";

const INCLUDE_PRELUDE: &'static str =
    "#![feature(const_fn_trait_bound)]mod fe_prelude;use fe_prelude::*;";

pub fn generate_rust(mut syntax_tree: SyntaxTree) -> Result<String, Error> {
    println!("Building Rust From:\n{:?}\n", syntax_tree);

    let mut rs = String::from(INCLUDE_PRELUDE);

    syntax_tree.imports.reverse();
    while let Some(import) = syntax_tree.imports.pop() {
        rs.push_str(&generate_import(&mut syntax_tree, import)?);
    }

    syntax_tree.items.reverse();
    while let Some(item) = syntax_tree.items.pop() {
        match item {
            ItemNode::Function(function) => {
                rs.push_str(&generate_function(&mut syntax_tree, function)?);
            }
        }
    }

    return Ok(rs);
}

pub fn generate_import(_syntax_tree: &mut SyntaxTree, import: ImportNode) -> Result<String, Error> {
    let mut import_rs = format!(
        "mod {}{};use {}{}::",
        IMPORT_PREFIX, import.source_token, IMPORT_PREFIX, import.source_token
    );

    match import.assignment {
        ImportAssignmentNode::Destructured(mut destructure) => {
            import_rs.push_str("{");

            destructure.fields.reverse();
            while let Some(field) = destructure.fields.pop() {
                if let Some(alias) = field.alias {
                    import_rs.push_str(&format!("{}:{},", field.field_token, alias.name_token));
                } else {
                    import_rs.push_str(&format!("{},", field.field_token));
                }
            }

            import_rs.push_str("};");
        }
        node => todo!("Unexpected node: {:?}", node),
    }

    return Ok(import_rs);
}

pub fn generate_function(
    mut syntax_tree: &mut SyntaxTree,
    mut function: FunctionNode,
) -> Result<String, Error> {
    let mut function_rs = String::new();

    if function.signature.is_public {
        function_rs.push_str("pub ");
    }

    function_rs.push_str(&format!("fn {}(", function.signature.name_token));

    function.signature.params.reverse();
    while let Some(param) = function.signature.params.pop() {
        function_rs.push_str(&format!(
            "{}:{}{}{}",
            param.name_token,
            if param.is_mutable { "mut " } else { "" },
            if param.is_borrowed { "&" } else { "" },
            fix_type_token(param.type_token),
        ));
    }

    function_rs.push_str(")");
    function_rs.push_str("{");

    function.body.statements.reverse();
    while let Some(statement) = function.body.statements.pop() {
        function_rs.push_str(&generate_statement(&mut syntax_tree, statement)?);
    }

    function_rs.push_str("}");

    return Ok(function_rs);
}

pub fn generate_statement(
    mut syntax_tree: &mut SyntaxTree,
    statement: StatementNode,
) -> Result<String, Error> {
    let mut statement_rs;

    match statement {
        StatementNode::Assignment(assignment) => {
            statement_rs = generate_assignment(&mut syntax_tree, assignment)?;
        }
        StatementNode::Expression(expression) => {
            statement_rs = generate_expression(&mut syntax_tree, expression, None)?;
        }
        node => todo!("Unexpected node: {:?}", node),
    }

    return Ok(statement_rs);
}

pub fn generate_assignment(
    mut syntax_tree: &mut SyntaxTree,
    assignment: AssignmentNode,
) -> Result<String, Error> {
    let mut assignment_rs = String::from("let ");

    assignment_rs.push_str(if assignment.left.reassignable {
        "mut "
    } else {
        ""
    });

    assignment_rs.push_str(&assignment.left.name_token);

    if let Some(type_node) = assignment.left.explicit_type.clone() {
        assignment_rs.push_str(":");

        assignment_rs.push_str(&generate_type(&mut syntax_tree, type_node)?);
    }

    assignment_rs.push_str("=");

    let expression_rs = generate_expression(
        &mut syntax_tree,
        assignment.right,
        assignment.left.explicit_type,
    )?;

    assignment_rs.push_str(&expression_rs);

    assignment_rs.push_str(";");

    return Ok(assignment_rs);
}

pub fn generate_type(
    mut syntax_tree: &mut SyntaxTree,
    type_node: TypeNode,
) -> Result<String, Error> {
    let mut type_rs = String::new();

    match type_node {
        TypeNode::Primative(PrimativeTypeNode {
            name_token,
            is_borrowed,
            is_optional,
        }) => {
            let name_token = match name_token.as_str() {
                "boolean" => "bool",
                "uint8" | "byte" => "u8",
                "uint16" => "u16",
                "uint32" => "u32",
                "uint64" => "u64",
                "uint128" => "u128",
                "biguint" => "u128",
                "uint" => "usize",
                "int8" => "i8",
                "int16" => "i16",
                "int32" => "i32",
                "int64" => "i64",
                "int128" => "i128",
                "bigint" => "i128",
                "int" => "isize",
                "float32" => "f32",
                "float64" => "f64",
                "char" => "char",
                "string" => "FeString",
                name => todo!("Unexpected primative type: {:?}", name),
            };

            if is_optional {
                type_rs.push_str("Option<");
            }

            if is_borrowed {
                type_rs.push_str("&");
            }

            type_rs.push_str(&name_token);

            if is_optional {
                type_rs.push_str(">");
            }
        }
        TypeNode::Structure(StructureTypeNode {
            path,
            generics,
            is_mutable,
            is_borrowed,
            is_optional,
        }) => {
            let path =
                if path.segments.len() == 1 && path.segments.get(0).unwrap().name_token == "list" {
                    StructureTypePathNode {
                        segments: vec![StructureTypePathSegmentNode {
                            name_token: String::from("Vec"),
                        }],
                    }
                } else {
                    path
                };

            if is_optional {
                type_rs.push_str("Option<");
            }

            if is_borrowed {
                type_rs.push_str("&");
            }

            if is_mutable {
                type_rs.push_str("mut ");
            }

            let path_rs = path
                .segments
                .into_iter()
                .map(|segment| segment.name_token)
                .collect::<Vec<String>>()
                .join("::");

            type_rs.push_str(&path_rs);

            if let Some(segments) = generics {
                type_rs.push_str("<");

                let segments_rs = segments
                    .into_iter()
                    .map(|segment| generate_type(&mut syntax_tree, segment).unwrap())
                    .collect::<Vec<String>>()
                    .join(",");

                type_rs.push_str(&segments_rs);

                type_rs.push_str(">");
            }

            if is_optional {
                type_rs.push_str(">");
            }
        }
        TypeNode::Tuple(TupleTypeNode {
            segments,
            is_optional,
        }) => {
            if is_optional {
                type_rs.push_str("Option<(");
            } else {
                type_rs.push_str("(");
            }

            let segments_rs = segments
                .into_iter()
                .map(|segment| generate_type(&mut syntax_tree, segment).unwrap())
                .collect::<Vec<String>>()
                .join(",");

            type_rs.push_str(&segments_rs);

            if is_optional {
                type_rs.push_str(")>");
            } else {
                type_rs.push_str(")");
            }
        }
        node => todo!("Unexpected node: {:?}", node),
    }

    return Ok(type_rs);
}

pub fn generate_expression(
    mut syntax_tree: &mut SyntaxTree,
    expression: ExpressionNode,
    explicit_type: Option<TypeNode>,
) -> Result<String, Error> {
    let mut expression_rs = String::new();

    match expression {
        ExpressionNode::Call(mut call) => {
            call.subject.segments.reverse();
            while let Some(segment) = call.subject.segments.pop() {
                match segment {
                    ExpressionCallPathSegmentNode::TypeIdentity(name) => {
                        expression_rs.push_str(&format!("{}::", name));
                    }
                    ExpressionCallPathSegmentNode::FunctionIdentity(name) => {
                        expression_rs.push_str(&name);
                    }
                    node => todo!("Unexpected node: {:?}", node),
                }
            }

            expression_rs.push_str("(");

            call.args.reverse();
            while let Some(arg) = call.args.pop() {
                expression_rs.push_str(&generate_expression(&mut syntax_tree, arg, None)?);

                if call.args.len() > 0 {
                    expression_rs.push_str(", ");
                }
            }

            expression_rs.push_str(");");
        }
        ExpressionNode::InstanceAccess(InstanceAccessNode { left, right }) => {
            let left_rs = generate_expression(&mut syntax_tree, *left, None)?;

            expression_rs.push_str(&format!("{}.", left_rs));

            match right {
                InstanceAccessRightNode::Call(ExpressionCallNode { subject, args }) => {
                    todo!();
                }
                InstanceAccessRightNode::Access(access) => {
                    todo!();
                }
                InstanceAccessRightNode::Reference(InstanceReferenceNode {
                    name_token,
                    is_borrowed,
                    is_mutable,
                }) => {
                    if is_borrowed {
                        expression_rs.push_str("&");
                    }

                    if is_mutable {
                        expression_rs.push_str("mut ");
                    }

                    expression_rs.push_str(&name_token);
                }
            };
        }
        ExpressionNode::InstanceReference(instance) => match instance.name_token.as_str() {
            "none" => expression_rs.push_str("None"),
            "some" => expression_rs.push_str("Some"),
            name_token => expression_rs.push_str(&name_token),
        },
        ExpressionNode::Literal(literal) => match literal {
            LiteralDataNode::Boolean(value) => {
                expression_rs.push_str(&format!("{}", value));
            }
            LiteralDataNode::Integer(value) => {
                let value = match explicit_type {
                    Some(TypeNode::Primative(PrimativeTypeNode { name_token, .. }))
                        if name_token.as_str() == "float32" || name_token.as_str() == "float64" =>
                    {
                        format!("{}.0", value)
                    }
                    _ => value,
                };

                expression_rs.push_str(&format!("{}", value));
            }
            LiteralDataNode::Char(value) => {
                expression_rs.push_str(&format!("'{}'", value));
            }
            LiteralDataNode::PlainString(value) => {
                expression_rs.push_str(&format!("FeString::from_slice(\"{}\")", value));
            }
            LiteralDataNode::TemplateString(mut template_string) => {
                let mut string = template_string.start_token;

                template_string.middle_tokens.reverse();
                while let Some(middle) = template_string.middle_tokens.pop() {
                    string.push_str("{}");
                    string.push_str(&middle);
                }

                string.push_str("{}");
                string.push_str(&template_string.end_token);

                let mut args = String::new();

                template_string.expressions.reverse();
                while let Some(expression) = template_string.expressions.pop() {
                    args.push_str(&generate_expression(&mut syntax_tree, expression, None)?);

                    if template_string.expressions.len() > 0 {
                        args.push_str(",");
                    }
                }

                expression_rs.push_str(&format!(
                    "FeString::from_owned(format!(\"{}\",{}))",
                    string, args
                ));
            }
            LiteralDataNode::Tuple(TupleNode { mut segments }) => {
                segments.reverse();

                expression_rs.push_str("(");

                if let Some(segment) = segments.pop() {
                    let segment_rs = generate_expression(&mut syntax_tree, segment, None)?;
                    expression_rs.push_str(&segment_rs);
                }

                while let Some(segment) = segments.pop() {
                    let segment_rs = generate_expression(&mut syntax_tree, segment, None)?;
                    expression_rs.push_str(&format!(",{}", segment_rs));
                }

                expression_rs.push_str(")");
            }
            LiteralDataNode::List(list) => match list {
                ListNode::Segmented(SegmentedListNode { mut segments }) => {
                    expression_rs.push_str("vec![");

                    segments.reverse();

                    if let Some(segment) = segments.pop() {
                        let segment_rs = generate_expression(&mut syntax_tree, segment, None)?;
                        expression_rs.push_str(&segment_rs);
                    }

                    while let Some(segment) = segments.pop() {
                        let segment_rs = generate_expression(&mut syntax_tree, segment, None)?;
                        expression_rs.push_str(&format!(",{}", segment_rs));
                    }

                    expression_rs.push_str("]");
                }
                ListNode::ForIn(ForInListNode {
                    for_name_token,
                    expression,
                    range,
                }) => {
                    todo!();
                }
            },
            node => todo!("Unexpected node: {:?}", node),
        },
        node => todo!("Unexpected node: {:?}", node),
    }

    return Ok(expression_rs);
}

fn fix_type_token(type_token: String) -> String {
    match type_token.as_str() {
        "string" => "FeString".to_string(),
        _ => type_token,
    }
}
