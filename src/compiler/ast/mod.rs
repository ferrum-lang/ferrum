use super::{tokens::Token, Error};

pub fn build_from_tokens(mut tokens: Vec<Token>) -> Result<Ast, Error> {
  // println!("{:?}\n", tokens);

  tokens.reverse();

  let mut ast = Ast::new();

  let mut is_public = false;

  while let Some(token) = tokens.pop() {
    match token {
      Token::Import => build_from_import(&mut tokens, &mut ast, token)?,
      Token::Function => build_from_function(&mut tokens, &mut ast, token, is_public)?,
      Token::Public => {
        is_public = true;
        continue;
      }
      _ => todo!("Unexpected token: {:?}\n\nAST: {:?}", token, ast),
    }

    is_public = false;
  }

  return Ok(ast);
}

fn build_from_import(tokens: &mut Vec<Token>, ast: &mut Ast, _: Token) -> Result<(), Error> {
  let token = tokens.pop().expect("Unfinished import!");

  match token {
    Token::DestructureOpenBrace => {
      let mut fields = vec![];

      loop {
        let token = tokens.pop().expect("Unfinished import!");

        match token {
          Token::DestructureField(field) => {
            let token = tokens.last().expect("Unfinished import!");

            match token {
              Token::DestructureAliasColon => {
                tokens.pop();

                match tokens.pop().expect("Unfinished import!") {
                  Token::DestructureAliasName(alias) => {
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
          Token::DestructureComma => {}
          Token::DestructureCloseBrace => {
            break;
          }
          _ => todo!("Unexpected token: {:?}", token),
        }
      }

      let assignment = ImportAssignmentNode::Destructured(DestructureAssignmentNode { fields });

      match tokens.pop().expect("Unfinished import!") {
        Token::ImportFrom => match tokens.pop().expect("Unfinished import!") {
          Token::ImportSource(source) => {
            ast.imports.push(ImportNode {
              assignment,
              source_token: source,
            });
          }
          token => todo!("Unexpected token: {:?}", token),
        },
        token => todo!("Unexpected token: {:?}", token),
      }

      match tokens.pop().expect("Unfinished import!") {
        Token::Semicolon => {}
        token => todo!("Unexpected token: {:?}", token),
      }
    }
    _ => todo!("Unexpected token: {:?}", token),
  }

  return Ok(());
}

fn build_from_function(
  mut tokens: &mut Vec<Token>,
  mut ast: &mut Ast,
  _: Token,
  is_public: bool,
) -> Result<(), Error> {
  let function_name;

  loop {
    let token = tokens.pop().expect("Unfinished function!");

    match token {
      Token::FunctionName(name) => {
        function_name = name;
        break;
      }
      _ => todo!("Unexpected token: {:?}", token),
    }
  }

  match tokens.pop().expect("Unfinished function!") {
    Token::FunctionParamsOpenParenthesis => {}
    token => todo!("Unexpected token: {:?}", token),
  }

  let mut params = vec![];

  loop {
    let token = tokens.pop().expect("Unfinished function!");

    match token {
      Token::FunctionParamsCloseParenthesis => {
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
    Token::FunctionExpressionsOpenBrace => {}
    token => todo!("Unexpected token: {:?}", token),
  }

  let mut statements = vec![];

  loop {
    let token = tokens.pop().expect("Unfinished function!");

    match token {
      Token::TypeAccessName(type_access_name) => {
        let mut segments = vec![ExpressionCallPathSegmentNode::TypeIdentity(
          type_access_name,
        )];

        match tokens.pop().expect("Unfinished function!") {
          Token::TypeAccessDoubleSemicolon => {}
          token => todo!("Unexpected token: {:?}", token),
        }

        loop {
          let token = tokens.pop().expect("Unfinished function!");

          match token {
            Token::FunctionCallName(function_call_name) => {
              segments.push(ExpressionCallPathSegmentNode::FunctionIdentity(
                function_call_name,
              ));

              let call_path = ExpressionCallPathNode { segments };

              let mut args = vec![];

              match tokens.pop().expect("Unfinished function!") {
                Token::FunctionCallOpenParenthesis => {}
                token => todo!("Unexpected token: {:?}", token),
              }

              loop {
                let token = tokens.pop().expect("Unfinished function!");

                match token {
                  Token::FunctionCallCloseParenthesis => {
                    break;
                  }
                  token => args.push(build_expression_node(&mut tokens, &mut ast, token)?),
                }
              }

              match tokens.pop().expect("Unfinished function!") {
                Token::Semicolon => {}
                token => todo!("Unexpected token: {:?}", token),
              }

              statements.push(StatementNode::Expression(ExpressionNode::Call(
                ExpressionCallNode {
                  subject: call_path,
                  args,
                },
              )));

              break;
            }
            token => todo!("Unexpected token: {:?}", token),
          }
        }
      }
      Token::FunctionExpressionsCloseBrace => {
        break;
      }
      token => todo!("Unexpected token: {:?}", token),
    }
  }

  let body = FunctionBodyNode { statements };

  ast
    .items
    .push(ItemNode::Function(FunctionNode { signature, body }));

  return Ok(());
}

fn build_expression_node(
  mut tokens: &mut Vec<Token>,
  mut ast: &mut Ast,
  token: Token,
) -> Result<ExpressionNode, Error> {
  match token {
    Token::Int(value) => {
      return Ok(ExpressionNode::Literal(LiteralDataNode::Integer(value)));
    }
    Token::PlainString(value) => {
      return Ok(ExpressionNode::Literal(LiteralDataNode::PlainString(value)));
    }
    Token::TemplateStringStart(start) => {
      let mut middle_tokens = vec![];
      let mut expressions = vec![];

      loop {
        let token = tokens.pop().expect("Unfinished expression!");

        match token {
          Token::TemplateStringMiddle(middle) => {
            middle_tokens.push(middle);
          }
          Token::TemplateStringEnd(end) => {
            return Ok(ExpressionNode::Literal(LiteralDataNode::TemplateString(
              TemplateStringNode {
                start_token: start,
                middle_tokens,
                expressions,
                end_token: end,
              },
            )))
          }
          Token::TemplateStringTemplateOpenBrace => {
            let token = tokens.pop().expect("Unfinished expression!");
            let expression = build_expression_node(&mut tokens, &mut ast, token)?;

            expressions.push(expression);

            match tokens.pop().expect("Unfinished expression!") {
              Token::TemplateStringTemplateCloseBrace => {}
              token => todo!("Unexpected token: {:?}", token),
            }
          }
          token => todo!("Unexpected token: {:?}", token),
        }
      }
    }
    token => todo!("Unexpected token: {:?}", token),
  }
}

#[derive(Debug)]
pub struct Ast {
  pub imports: Vec<ImportNode>,
  pub items: Vec<ItemNode>,
}

impl Ast {
  fn new() -> Self {
    Self {
      imports: vec![],
      items: vec![],
    }
  }
}

#[derive(Debug)]
pub struct ImportNode {
  pub assignment: ImportAssignmentNode,
  pub source_token: String,
}

#[derive(Debug)]
pub enum ImportAssignmentNode {
  Destructured(DestructureAssignmentNode),
}

#[derive(Debug)]
pub struct DestructureAssignmentNode {
  pub fields: Vec<DestructureAssignmentFieldNode>,
}

#[derive(Debug)]
pub struct DestructureAssignmentFieldNode {
  pub field_token: String,
  pub alias: Option<DestructureAssignmentFieldAliasNode>,
}

#[derive(Debug)]
pub struct DestructureAssignmentFieldAliasNode {
  pub name_token: String,
}

#[derive(Debug)]
pub enum ItemNode {
  Function(FunctionNode),
}

#[derive(Debug)]
pub struct FunctionNode {
  pub signature: FunctionSignatureNode,
  pub body: FunctionBodyNode,
}

#[derive(Debug)]
pub struct FunctionSignatureNode {
  pub is_public: bool,
  pub name_token: String,
  pub params: Vec<FunctionParamNode>,
  pub return_type: Option<ReturnTypeNode>,
}

#[derive(Debug)]
pub struct FunctionParamNode {}

#[derive(Debug)]
pub struct ReturnTypeNode {}

#[derive(Debug)]
pub struct FunctionBodyNode {
  pub statements: Vec<StatementNode>,
}

#[derive(Debug)]
pub enum StatementNode {
  Expression(ExpressionNode),
}

#[derive(Debug)]
pub enum ExpressionNode {
  Call(ExpressionCallNode),
  Literal(LiteralDataNode),
}

#[derive(Debug)]
pub struct ExpressionCallNode {
  pub subject: ExpressionCallPathNode,
  pub args: Vec<ExpressionNode>,
}

#[derive(Debug)]
pub struct ExpressionCallPathNode {
  pub segments: Vec<ExpressionCallPathSegmentNode>,
}

#[derive(Debug)]
pub enum ExpressionCallPathSegmentNode {
  TypeIdentity(String),
  FunctionIdentity(String),
}

#[derive(Debug)]
pub enum LiteralDataNode {
  PlainString(String),
  TemplateString(TemplateStringNode),
  Integer(String),
}

#[derive(Debug)]
pub struct TemplateStringNode {
  pub start_token: String,
  pub middle_tokens: Vec<String>,
  pub expressions: Vec<ExpressionNode>,
  pub end_token: String,
}
