use super::*;

pub fn translate_def_fn(ast: &mut SemanticAST, def_fn: p::DefFn) -> Result<()> {
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
        block: Box::new(map_fn_impl(def_fn.r#impl)?),
    };

    ast.items.push(Item::Fn(item));

    return Ok(());
}

fn map_fn_impl(fn_impl: p::DefFnImpl) -> Result<Block> {
    let statements = match fn_impl {
        p::DefFnImpl::Block(block) => {
            let mut statements = vec![];
            
            for statement in block.statements.into_iter() {
                statements.push(map_statement(statement)?);
            }

            statements
        },
        p::DefFnImpl::Expression(expr) => vec![Statement::Semi(map_expr(expr)?)],
    };

    return Ok(Block { statements });
}

fn map_statement(statement: p::Statement) -> Result<Statement> {
    let statement = match statement {
        p::Statement::Assignment(assign) => todo!(),
        p::Statement::Expression(expr) => Statement::Semi(map_expr(expr)?),
    };

    return Ok(statement);
}

fn map_expr(expr: p::Expression) -> Result<Expr> {
    let expr = match expr {
        p::Expression::FunctionCall(call) => Expr::Call(ExprCall {
            func: Box::new(Expr::Path(ExprPath {
                path: Path {
                    segments: vec![
                        PathSegment {
                            ident: call.name,
                            arguments: PathArguments::None,
                        },
                    ],
                },
            })),
            args: call.args
                .into_iter()
                .map(|arg| map_expr(*arg.value).unwrap())
                .collect(),
        }),
        p::Expression::Literal(literal) => match literal {
            p::Literal::Bool(value) => Expr::Lit(ExprLit { literal: Literal::Bool(value) }),
            p::Literal::String(value) => match value {
                 p::LiteralString::Plain(value) => Expr::Lit(ExprLit { literal: Literal::Str(value) }),
                 _ => todo!(),
            },
            p::Literal::Number(p::LiteralNumber { value }) => Expr::Lit(ExprLit { literal: Literal::Float(value) }),
            _ => todo!(),
        },
        _ => todo!(),
    };

    return Ok(expr);
}

