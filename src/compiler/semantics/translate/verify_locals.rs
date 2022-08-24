use super::*;

use std::collections::HashMap;

use anyhow::Result;

pub fn verify_locals(ast: &p::AST) -> Result<()> {
    for node in ast.nodes.iter() {
        match node {
            p::RootNode::Definition(def) => match def {
                p::Definition::Function(def_fn) => verify_locals_fn(def_fn)?,
                p::Definition::Type(def_type) => match def_type {
                    p::DefType::Class(def_class) => verify_locals_class(def_class)?,
                    _ => {},
                },
            },
            _ => unreachable!("No top-level statements by this point"),
        }
    }

    return Ok(());
}

type Locals = HashMap<String, Local>;
struct Local {
    pub is_reassign: bool,
    pub is_mut: bool,
}

fn verify_locals_fn(def_fn: &p::DefFn) -> Result<()> {
    let mut locals = HashMap::new();

    for param in def_fn.signature.params.iter() {
        locals.insert(param.name.clone(), Local {
            is_reassign: false,
            is_mut: param.is_mutable,
        });
    }

    match &def_fn.r#impl {
        p::DefFnImpl::Expression(expr) => verify_locals_expr(&mut locals, expr)?,
        p::DefFnImpl::Block(block) => verify_locals_block(&mut locals, block)?,
    }

    return Ok(());
}

fn verify_locals_class(def_class: &p::DefClass) -> Result<()> {
    for method in def_class.methods.iter() {
        let mut locals = HashMap::new();

        locals.insert("self".to_string(), Local {
            is_reassign: false,
            is_mut: method.signature.is_mut,
        });

        for param in method.signature.params.iter() {
            locals.insert(param.name.clone(), Local {
                is_reassign: false,
                is_mut: param.is_mutable,
            });
        }

        match &method.r#impl {
            p::DefFnImpl::Expression(expr) => verify_locals_expr(&mut locals, expr)?,
            p::DefFnImpl::Block(block) => verify_locals_block(&mut locals, block)?,
        }
    }

    for r#impl in def_class.impls.iter() {
        for method in r#impl.methods.iter() {
            let mut locals = HashMap::new();

            locals.insert("self".to_string(), Local {
                is_reassign: false,
                is_mut: method.signature.is_mut,
            });

            for param in method.signature.params.iter() {
                locals.insert(param.name.clone(), Local {
                    is_reassign: false,
                    is_mut: param.is_mutable,
                });
            }

            match &method.r#impl {
                p::DefFnImpl::Expression(expr) => verify_locals_expr(&mut locals, expr)?,
                p::DefFnImpl::Block(block) => verify_locals_block(&mut locals, block)?,
            }
        }
    }

    return Ok(());
}

fn verify_locals_block(locals: &mut Locals, block: &p::Block) -> Result<()> {
    for statement in block.statements.iter() {
        match statement {
            p::Statement::Assignment(assignment) => {
                if let Some(local) = &assignment.local_var {
                    let name = match &assignment.target {
                        p::AssignmentTarget::Reference(reference) => {
                            if reference.receiver.is_some() {
                                todo!("Unexpected assignment-target receiver on local var");
                            }

                            reference.name.clone()
                        },
                        p::AssignmentTarget::Direct(direct) => direct.clone(),
                        p::AssignmentTarget::DestructureObject(destruct_obj) => todo!(),
                        p::AssignmentTarget::DestructureTuple(destruct_tuple) => todo!(),
                    };

                    locals.insert(name, Local {
                        is_reassign: *local == p::AssignmentLocalVar::Let,
                        is_mut: match &assignment.expression {
                            Some(expr) => match expr {
                                p::Expression::Mut(_) => true,
                                _ => false,
                            },
                            _ => todo!(),
                        },
                    });
                } else {
                    match &assignment.target {
                        p::AssignmentTarget::Reference(reference) => {
                            if let Some(receiver) = &reference.receiver {
                                todo!();
                            } else if let Some(local) = locals.get(&reference.name) {
                                if !local.is_reassign {
                                    panic!("Cannot reassign!");
                                }
                            } else {
                                panic!("Variable name not found!");
                            }
                        },
                        p::AssignmentTarget::Direct(name) => {
                            if let Some(local) = locals.get(name) {
                                if !local.is_reassign {
                                    panic!("Cannot reassign!");
                                }
                            } else {
                                panic!("Variable name not found!");
                            }
                        }
                        _ => todo!(),
                    }
                }
            },
            p::Statement::Expression(expr) => verify_locals_expr(locals, &expr)?,
        }
    }

    return Ok(());
}

fn verify_locals_expr(locals: &mut Locals, expr: &p::Expression) -> Result<()> {
    match expr {
        p::Expression::MethodCall(method_call) => todo!(),
        // _ => todo!("check for mutable method call");
        _ => {},
    }

    return Ok(());
}

