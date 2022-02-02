use super::{symbols::Symbol, syntax::*, Error};

pub fn parse_symbols(mut tokens: Vec<Symbol>) -> Result<SyntaxTree, Error> {
  // println!("Building Syntax Tree From:\n{:?}\n", tokens);

  tokens.reverse();

  let mut syntax_tree = SyntaxTree::new();

  let mut is_public = false;

  while let Some(token) = tokens.pop() {
    match token {
      Symbol::Import => parse_import(&mut tokens, &mut syntax_tree, token)?,
      Symbol::Function => parse_function(&mut tokens, &mut syntax_tree, token, is_public)?,
      Symbol::Public => {
        is_public = true;
        continue;
      }
      _ => todo!(
        "Unexpected token: {:?}\n\nSyntax Tree: {:?}",
        token,
        syntax_tree
      ),
    }

    is_public = false;
  }

  return Ok(syntax_tree);
}

fn parse_import(
  tokens: &mut Vec<Symbol>,
  syntax_tree: &mut SyntaxTree,
  _: Symbol,
) -> Result<(), Error> {
  let token = tokens.pop().expect("Unfinished import!");

  match token {
    Symbol::DestructureOpenBrace => {
      let mut fields = vec![];

      loop {
        let token = tokens.pop().expect("Unfinished import!");

        match token {
          Symbol::DestructureField(field) => {
            let token = tokens.last().expect("Unfinished import!");

            match token {
              Symbol::DestructureAliasColon => {
                tokens.pop();

                match tokens.pop().expect("Unfinished import!") {
                  Symbol::DestructureAliasName(alias) => {
                    fields.push(DestructureAssignmentFieldNode {
                      field_token: field,
                      alias: Some(DestructureAssignmentFieldAliasNode { name_token: alias }),
                    });
                  }
                  token => todo!("Unexpected token: {:?}", token),
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
          _ => todo!("Unexpected token: {:?}", token),
        }
      }

      let assignment = ImportAssignmentNode::Destructured(DestructureAssignmentNode { fields });

      match tokens.pop().expect("Unfinished import!") {
        Symbol::ImportFrom => match tokens.pop().expect("Unfinished import!") {
          Symbol::ImportSource(source) => {
            syntax_tree.imports.push(ImportNode {
              assignment,
              source_token: source,
            });
          }
          token => todo!("Unexpected token: {:?}", token),
        },
        token => todo!("Unexpected token: {:?}", token),
      }

      match tokens.pop().expect("Unfinished import!") {
        Symbol::Semicolon => {}
        token => todo!("Unexpected token: {:?}", token),
      }
    }
    _ => todo!("Unexpected token: {:?}", token),
  }

  return Ok(());
}

fn parse_function(
  mut tokens: &mut Vec<Symbol>,
  mut syntax_tree: &mut SyntaxTree,
  _: Symbol,
  is_public: bool,
) -> Result<(), Error> {
  let function_name;

  loop {
    let token = tokens.pop().expect("Unfinished function!");

    match token {
      Symbol::FunctionName(name) => {
        function_name = name;
        break;
      }
      _ => todo!("Unexpected token: {:?}", token),
    }
  }

  match tokens.pop().expect("Unfinished function!") {
    Symbol::FunctionParamsOpenParenthesis => {}
    token => todo!("Unexpected token: {:?}", token),
  }

  let mut params = vec![];

  loop {
    let token = tokens.pop().expect("Unfinished function!");

    match token {
      Symbol::FunctionParamsParamName(_) => {
        params.push(build_function_param_node(
          &mut tokens,
          &mut syntax_tree,
          token,
        )?);
      }
      Symbol::FunctionParamsCloseParenthesis => {
        break;
      }
      token => todo!("Unexpected token: {:?}", token),
    }
  }

  let signature = FunctionSignatureNode {
    is_public,
    name_token: function_name,
    params,
    return_type: None,
  };

  match tokens.pop().expect("Unfinished function!") {
    Symbol::FunctionExpressionsOpenBrace => {}
    token => todo!("Unexpected token: {:?}", token),
  }

  let mut statements = vec![];

  loop {
    let token = tokens.pop().expect("Unfinished function!");

    match token {
      Symbol::FunctionExpressionsCloseBrace => {
        break;
      }
      token => statements.push(build_statement_node(&mut tokens, &mut syntax_tree, token)?),
    }
  }

  let body = FunctionBodyNode { statements };

  syntax_tree
    .items
    .push(ItemNode::Function(FunctionNode { signature, body }));

  return Ok(());
}

fn build_function_param_node(
  mut tokens: &mut Vec<Symbol>,
  mut syntax_tree: &mut SyntaxTree,
  token: Symbol,
) -> Result<FunctionParamNode, Error> {
  match token {
    Symbol::FunctionParamsParamName(param_name) => {
      match tokens.pop().expect("Unfinished function param!") {
        Symbol::FunctionParamsParamTypeColon => {}
        token => todo!("Unexpected token: {:?}", token),
      };

      let is_mutable = match tokens.last().expect("Unfinished function param!") {
        Symbol::FunctionParamsParamTypeMutable => {
          tokens.pop();
          true
        }
        _ => false,
      };

      let is_borrowed = match tokens.last().expect("Unfinished function param!") {
        Symbol::FunctionParamsParamTypeBorrowed => {
          tokens.pop();
          true
        }
        _ => false,
      };

      let type_name = match tokens.pop().expect("Unfinished function param!") {
        Symbol::FunctionParamsParamTypeName(name) => name,
        token => todo!("Unexpected token: {:?}", token),
      };

      return Ok(FunctionParamNode {
        name_token: param_name,
        is_mutable,
        is_borrowed,
        type_token: type_name,
      });
    }
    token => todo!("Unexpected token: {:?}", token),
  }
}

fn build_statement_node(
  mut tokens: &mut Vec<Symbol>,
  mut syntax_tree: &mut SyntaxTree,
  token: Symbol,
) -> Result<StatementNode, Error> {
  match token {
    Symbol::TypeAccessName(_) | Symbol::FunctionCallName(_) => {
      let node =
        StatementNode::Expression(build_expression_node(&mut tokens, &mut syntax_tree, token)?);

      match tokens.pop().expect("Unfinished expression!") {
        Symbol::Semicolon => {}
        token => todo!("Unexpected token: {:?}", token),
      }

      return Ok(node);
    }
    Symbol::Const => match tokens.pop().expect("Unfinished expression!") {
      Symbol::VariableName(variable_name) => {
        match tokens.pop().expect("Unfinished expression!") {
          Symbol::Assignment => {}
          token => todo!("Unexpected token: {:?}", token),
        }

        let token = tokens.pop().expect("Unfinised expression!");

        let node = StatementNode::Assignment(AssignmentNode {
          left: AssignmentLeftNode {
            reassignable: false,
            name_token: variable_name,
          },
          right: build_expression_node(&mut tokens, &mut syntax_tree, token)?,
        });

        match tokens.pop().expect("Unfinished expression!") {
          Symbol::Semicolon => {}
          token => todo!("Unexpected token: {:?}", token),
        }

        return Ok(node);
      }
      token => todo!("Unexpected token: {:?}", token),
    },
    token => todo!("Unexpected token: {:?}", token),
  }
}

fn build_expression_node(
  mut tokens: &mut Vec<Symbol>,
  mut syntax_tree: &mut SyntaxTree,
  token: Symbol,
) -> Result<ExpressionNode, Error> {
  match token {
    Symbol::Int(value) => {
      return Ok(ExpressionNode::Literal(LiteralDataNode::Integer(value)));
    }
    Symbol::PlainString(value) => {
      return Ok(ExpressionNode::Literal(LiteralDataNode::PlainString(value)));
    }
    Symbol::TemplateStringStart(start) => {
      let mut middle_tokens = vec![];
      let mut expressions = vec![];

      loop {
        let token = tokens.pop().expect("Unfinished expression!");

        match token {
          Symbol::TemplateStringMiddle(middle) => {
            middle_tokens.push(middle);
          }
          Symbol::TemplateStringEnd(end) => {
            return Ok(ExpressionNode::Literal(LiteralDataNode::TemplateString(
              TemplateStringNode {
                start_token: start,
                middle_tokens,
                expressions,
                end_token: end,
              },
            )))
          }
          Symbol::TemplateStringTemplateOpenBrace => {
            let token = tokens.pop().expect("Unfinished expression!");
            let expression = build_expression_node(&mut tokens, &mut syntax_tree, token)?;

            expressions.push(expression);

            match tokens.pop().expect("Unfinished expression!") {
              Symbol::TemplateStringTemplateCloseBrace => {}
              token => todo!("Unexpected token: {:?}", token),
            }
          }
          token => todo!("Unexpected token: {:?}", token),
        }
      }
    }
    Symbol::InstanceBorrow => match tokens.pop().expect("Unfinished expression!") {
      Symbol::InstanceReferenceName(instance_reference_name) => {
        return Ok(ExpressionNode::InstanceReference(InstanceReferenceNode {
          name_token: instance_reference_name,
          is_borrowed: true,
        }));
      }
      token => todo!("Unexpected token: {:?}", token),
    },
    Symbol::InstanceReferenceName(instance_reference_name) => {
      return Ok(ExpressionNode::InstanceReference(InstanceReferenceNode {
        name_token: instance_reference_name,
        is_borrowed: false,
      }));
    }
    Symbol::TypeAccessName(_) => {
      return Ok(ExpressionNode::Call(build_expression_call_node(
        &mut tokens,
        &mut syntax_tree,
        token,
        vec![],
      )?));
    }
    Symbol::FunctionCallName(_) => {
      return Ok(ExpressionNode::Call(build_expression_call_node(
        &mut tokens,
        &mut syntax_tree,
        token,
        vec![],
      )?));
    }
    token => todo!("Unexpected token: {:?}", token),
  }
}

fn build_expression_call_node(
  mut tokens: &mut Vec<Symbol>,
  mut syntax_tree: &mut SyntaxTree,
  token: Symbol,
  mut segments: Vec<ExpressionCallPathSegmentNode>,
) -> Result<ExpressionCallNode, Error> {
  match token {
    Symbol::TypeAccessName(type_access_name) => {
      segments.push(ExpressionCallPathSegmentNode::TypeIdentity(
        type_access_name,
      ));

      match tokens.pop().expect("Unfinished function!") {
        Symbol::TypeAccessDoubleSemicolon => {}
        token => todo!("Unexpected token: {:?}", token),
      }

      loop {
        let token = tokens.pop().expect("Unfinished function!");

        match token {
          Symbol::FunctionCallName(_) => {
            return build_expression_call_node(&mut tokens, &mut syntax_tree, token, segments);
          }
          token => todo!("Unexpected token: {:?}", token),
        }
      }
    }
    Symbol::FunctionCallName(function_call_name) => {
      segments.push(ExpressionCallPathSegmentNode::FunctionIdentity(
        function_call_name,
      ));

      let call_path = ExpressionCallPathNode { segments };

      let mut args = vec![];

      match tokens.pop().expect("Unfinished function!") {
        Symbol::FunctionCallOpenParenthesis => {}
        token => todo!("Unexpected token: {:?}", token),
      }

      loop {
        let token = tokens.pop().expect("Unfinished function!");

        match token {
          Symbol::FunctionCallCloseParenthesis => {
            break;
          }
          token => args.push(build_expression_node(&mut tokens, &mut syntax_tree, token)?),
        }
      }

      return Ok(ExpressionCallNode {
        subject: call_path,
        args,
      });
    }
    token => todo!("Unexpected token: {:?}", token),
  }
}
