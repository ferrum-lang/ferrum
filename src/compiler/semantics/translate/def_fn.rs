use super::*;

use std::collections::HashMap;

use quote::quote;

pub fn translate_def_fn(
    fn_params_map: &FnParamsMap,
    ast: &mut SemanticAST,
    def_fn: p::DefFn,
) -> Result<()> {
    let item = ItemFn {
        is_public: def_fn.signature.is_public,
        signature: FnSignature {
            is_const: false,
            is_async: def_fn.signature.is_async != p::MaybeBool::False,
            name: def_fn.signature.name,
            generics: Generics {
                params: vec![],
            },
            params: vec![],
            return_type: ReturnType::Default,
        },
        block: Box::new(map_fn_impl(fn_params_map, def_fn.r#impl)?),
    };

    ast.items.push(Item::Fn(item));

    return Ok(());
}

fn map_fn_impl(fn_params_map: &FnParamsMap, fn_impl: p::DefFnImpl) -> Result<Block> {
    let statements = match fn_impl {
        p::DefFnImpl::Block(block) => {
            let mut statements = vec![];
            
            for statement in block.statements.into_iter() {
                statements.push(map_statement(fn_params_map, statement)?);
            }

            statements
        },
        p::DefFnImpl::Expression(expr) => vec![Statement::Semi(map_expr(fn_params_map, expr)?)],
    };

    return Ok(Block { statements });
}

fn map_statement(fn_params_map: &FnParamsMap, statement: p::Statement) -> Result<Statement> {
    let statement = match statement {
        p::Statement::Assignment(assign) => todo!(),
        p::Statement::Expression(expr) => Statement::Semi(map_expr(fn_params_map, expr)?),
    };

    return Ok(statement);
}

fn map_expr(fn_params_map: &FnParamsMap, expr: p::Expression) -> Result<Expr> {
    let expr = match expr {
        p::Expression::Literal(literal) => Expr::Lit(ExprLit { literal: map_literal(literal)? }),
        p::Expression::FunctionCall(call) => map_expr_fn_call(fn_params_map, call)?,
        p::Expression::TemplateString(template_str) => map_expr_template_str(fn_params_map, template_str)?,
        _=> todo!("{expr:?}"),
    };

    return Ok(expr);
}

fn map_expr_fn_call(fn_params_map: &FnParamsMap, call: p::FunctionCall) -> Result<Expr> {
    let fn_params = fn_params_map.get(&call.name);

    let mut args = vec![];

    if let Some(fn_params) = fn_params {
        if call.args.len() > fn_params.len() {
            todo!("Too many args!");
        }

        let mut named_args = HashMap::new();
        let mut unnamed_args = vec![];

        for arg in call.args.clone().into_iter() {
            if let Some(name) = arg.name.clone() {
                let fn_param = fn_params.iter()
                    .find(|p| p.0 == name)
                    .expect("Couldn't find param");

                named_args.insert(fn_param.0.clone(), arg);
            } else {
                unnamed_args.push(arg);
            }
        }

        unnamed_args.reverse();

        for param in fn_params.clone().into_iter() {
            if let Some(named_arg) = named_args.remove(&param.0) {
                args.push(map_expr(&fn_params_map, *named_arg.value)?);
            } else if let Some(unnamed_arg) = unnamed_args.pop() {
                args.push(map_expr(&fn_params_map, *unnamed_arg.value)?);
            } else if let Some(default_val) = param.1 {
                args.push(map_expr(&fn_params_map, *default_val)?);
            } else {
                todo!("Missing arg!");
            }
        }
    } else {
        for arg in call.args.into_iter() {
            args.push(map_expr(fn_params_map, *arg.value)?);
        }
    }

    let mut segments = match call.receiver {
        Some(static_ref) => map_static_ref_to_path_segments(static_ref),
        None => vec![],
    };

    segments.push(PathSegment {
        ident: call.name,
        arguments: PathArguments::None,
    });

    let expr = Expr::Call(ExprCall {
        func: Box::new(Expr::Path(ExprPath {
            path: Path { segments },
        })),
        args,
    });

    return Ok(expr);
}

fn map_literal(literal: p::Literal) -> Result<Literal> {
    let literal = match literal {
        p::Literal::Bool(value) => Literal::Bool(value),
        p::Literal::PlainString(p::LiteralString { value }) => Literal::Str(value),
        p::Literal::Number(p::LiteralNumber { value }) => Literal::Float(value),
        _ => todo!(),
    };

    return Ok(literal);
}

fn map_expr_template_str(fn_params_map: &FnParamsMap, template_string: p::TemplateString) -> Result<Expr> {
    let mut format_str = template_string.start;
    let mut exprs = vec![];

    let mut requires_templating = false;

    for part in template_string.parts.into_iter() {
        match *part.expression {
            p::Expression::Literal(literal) => {
                let string = match literal {
                    p::Literal::Bool(value) => value.to_string(),
                    p::Literal::Number(p::LiteralNumber { value }) => value,
                    p::Literal::PlainString(p::LiteralString { value }) => value,
                    p::Literal::Char(p::LiteralChar { value }) => value,
                };

                format_str.push_str(&string);
            },
            expr => {
                requires_templating = true;

                format_str.push_str("{}");
                format_str.push_str(&part.post_string);
                exprs.push(map_expr(fn_params_map, expr)?);
            },
        }
    }

    if !requires_templating {
        return Ok(Expr::Lit(ExprLit {
            literal: Literal::Str(format_str),
        }));
    }

    let mut tokens = quote! { #format_str };

    for i in 0..exprs.len() {
        tokens.extend(quote! { , values.#i });
    }

    let expr = Expr::Block(Block {
        statements: vec![
            Statement::Local(Local {
                pattern: Pattern::Ident(PatIdent {
                    is_mut: false,
                    is_ref: false,
                    name: "values".to_string(),
                }),
                init: Some(Box::new(
                        Expr::Tuple(ExprTuple {
                            elems: exprs,
                        })
                )),
            }),
            Statement::Expr(
                Expr::Macro(ExprMacro { mac: Macro {
                    path: Path { segments: vec![PathSegment {
                        ident: "format".to_string(),
                        arguments: PathArguments::None,
                    }] },
                    delimiter: MacroDelimiter::Paren,
                    tokens,
                }})),
        ],
    });

    return Ok(expr);
}

fn map_static_ref_to_path_segments(static_ref: p::ReferenceStatic) -> Vec<PathSegment> {
    let mut segments = match static_ref.receiver {
        Some(static_ref) => map_static_ref_to_path_segments(*static_ref),
        None => vec![],
    };

    segments.push(PathSegment {
        ident: static_ref.name,
        arguments: PathArguments::None,
    });

    return segments;
}

