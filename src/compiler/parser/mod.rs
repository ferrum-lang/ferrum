use super::{symbols::Symbol, syntax::*, Error};

pub fn parse_symbols(mut symbols: Vec<Symbol>) -> Result<SyntaxTree, Error> {
  // println!("Building Syntax Tree From:\n{:?}\n", symbols);

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
                      alias: Some(DestructureAssignmentFieldAliasNode { name_token: alias }),
                    });
                  }
                  symbol => todo!("Unexpected symbol: {:?}", symbol),
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

      let assignment = ImportAssignmentNode::Destructured(DestructureAssignmentNode { fields });

      match symbols.pop().expect("Unfinished import!") {
        Symbol::ImportFrom => match symbols.pop().expect("Unfinished import!") {
          Symbol::ImportSource(source) => {
            syntax_tree.imports.push(ImportNode {
              assignment,
              source_token: source,
            });
          }
          symbol => todo!("Unexpected symbol: {:?}", symbol),
        },
        symbol => todo!("Unexpected symbol: {:?}", symbol),
      }

      match symbols.pop().expect("Unfinished import!") {
        Symbol::Semicolon => {}
        symbol => todo!("Unexpected symbol: {:?}", symbol),
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
    symbol => todo!("Unexpected symbol: {:?}", symbol),
  }

  let mut params = vec![];

  loop {
    let symbol = symbols.pop().expect("Unfinished function!");

    match symbol {
      Symbol::FunctionParamsParamName(_) => {
        params.push(build_function_param_node(
          &mut symbols,
          &mut syntax_tree,
          symbol,
        )?);
      }
      Symbol::FunctionParamsCloseParenthesis => {
        break;
      }
      symbol => todo!("Unexpected symbol: {:?}", symbol),
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
    symbol => todo!("Unexpected symbol: {:?}", symbol),
  }

  let mut statements = vec![];

  loop {
    let symbol = symbols.pop().expect("Unfinished function!");

    match symbol {
      Symbol::FunctionExpressionsCloseBrace => {
        break;
      }
      symbol => statements.push(build_statement_node(&mut symbols, &mut syntax_tree, symbol)?),
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
        symbol => todo!("Unexpected symbol: {:?}", symbol),
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
        symbol => todo!("Unexpected symbol: {:?}", symbol),
      };

      return Ok(FunctionParamNode {
        name_token: param_name,
        is_mutable,
        is_borrowed,
        type_token: type_name,
      });
    }
    symbol => todo!("Unexpected symbol: {:?}", symbol),
  }
}

fn build_statement_node(
  mut symbols: &mut Vec<Symbol>,
  mut syntax_tree: &mut SyntaxTree,
  symbol: Symbol,
) -> Result<StatementNode, Error> {
  match symbol {
    Symbol::TypeAccessName(_) | Symbol::FunctionCallName(_) => {
      let node =
        StatementNode::Expression(build_expression_node(&mut symbols, &mut syntax_tree, symbol)?);

      match symbols.pop().expect("Unfinished expression!") {
        Symbol::Semicolon => {}
        symbol => todo!("Unexpected symbol: {:?}", symbol),
      }

      return Ok(node);
    }
    Symbol::Const => match symbols.pop().expect("Unfinished expression!") {
      Symbol::VariableName(variable_name) => {
        match symbols.pop().expect("Unfinished expression!") {
          Symbol::Assignment => {}
          symbol => todo!("Unexpected symbol: {:?}", symbol),
        }

        let symbol = symbols.pop().expect("Unfinised expression!");

        let node = StatementNode::Assignment(AssignmentNode {
          left: AssignmentLeftNode {
            reassignable: false,
            name_token: variable_name,
          },
          right: build_expression_node(&mut symbols, &mut syntax_tree, symbol)?,
        });

        match symbols.pop().expect("Unfinished expression!") {
          Symbol::Semicolon => {}
          symbol => todo!("Unexpected symbol: {:?}", symbol),
        }

        return Ok(node);
      }
      symbol => todo!("Unexpected symbol: {:?}", symbol),
    },
    symbol => todo!("Unexpected symbol: {:?}", symbol),
  }
}

fn build_expression_node(
  mut symbols: &mut Vec<Symbol>,
  mut syntax_tree: &mut SyntaxTree,
  symbol: Symbol,
) -> Result<ExpressionNode, Error> {
  match symbol {
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
        let symbol = symbols.pop().expect("Unfinished expression!");

        match symbol {
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
            let symbol = symbols.pop().expect("Unfinished expression!");
            let expression = build_expression_node(&mut symbols, &mut syntax_tree, symbol)?;

            expressions.push(expression);

            match symbols.pop().expect("Unfinished expression!") {
              Symbol::TemplateStringTemplateCloseBrace => {}
              symbol => todo!("Unexpected symbol: {:?}", symbol),
            }
          }
          symbol => todo!("Unexpected symbol: {:?}", symbol),
        }
      }
    }
    Symbol::InstanceBorrow => match symbols.pop().expect("Unfinished expression!") {
      Symbol::InstanceReferenceName(instance_reference_name) => {
        return Ok(ExpressionNode::InstanceReference(InstanceReferenceNode {
          name_token: instance_reference_name,
          is_borrowed: true,
        }));
      }
      symbol => todo!("Unexpected symbol: {:?}", symbol),
    },
    Symbol::InstanceReferenceName(instance_reference_name) => {
      return Ok(ExpressionNode::InstanceReference(InstanceReferenceNode {
        name_token: instance_reference_name,
        is_borrowed: false,
      }));
    }
    Symbol::TypeAccessName(_) => {
      return Ok(ExpressionNode::Call(build_expression_call_node(
        &mut symbols,
        &mut syntax_tree,
        symbol,
        vec![],
      )?));
    }
    Symbol::FunctionCallName(_) => {
      return Ok(ExpressionNode::Call(build_expression_call_node(
        &mut symbols,
        &mut syntax_tree,
        symbol,
        vec![],
      )?));
    }
    symbol => todo!("Unexpected symbol: {:?}", symbol),
  }
}

fn build_expression_call_node(
  mut symbols: &mut Vec<Symbol>,
  mut syntax_tree: &mut SyntaxTree,
  symbol: Symbol,
  mut segments: Vec<ExpressionCallPathSegmentNode>,
) -> Result<ExpressionCallNode, Error> {
  match symbol {
    Symbol::TypeAccessName(type_access_name) => {
      segments.push(ExpressionCallPathSegmentNode::TypeIdentity(
        type_access_name,
      ));

      match symbols.pop().expect("Unfinished function!") {
        Symbol::TypeAccessDoubleSemicolon => {}
        symbol => todo!("Unexpected symbol: {:?}", symbol),
      }

      loop {
        let symbol = symbols.pop().expect("Unfinished function!");

        match symbol {
          Symbol::FunctionCallName(_) => {
            return build_expression_call_node(&mut symbols, &mut syntax_tree, symbol, segments);
          }
          symbol => todo!("Unexpected symbol: {:?}", symbol),
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
        symbol => todo!("Unexpected symbol: {:?}", symbol),
      }

      loop {
        let symbol = symbols.pop().expect("Unfinished function!");

        match symbol {
          Symbol::FunctionCallCloseParenthesis => {
            break;
          }
          symbol => args.push(build_expression_node(&mut symbols, &mut syntax_tree, symbol)?),
        }
      }

      return Ok(ExpressionCallNode {
        subject: call_path,
        args,
      });
    }
    symbol => todo!("Unexpected symbol: {:?}", symbol),
  }
}
