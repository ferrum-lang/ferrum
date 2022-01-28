use super::{tokens::Token, Error};

pub fn build_from_tokens(mut tokens: Vec<Token>) -> Result<Ast, Error> {
  println!("{:?}\n", tokens);

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
              import_assignment: assignment,
              import_source_token: source,
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
  tokens: &mut Vec<Token>,
  ast: &mut Ast,
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
                  Token::PlainString(value) => {
                    args.push(ExpressionCallArgNode::Literal(
                      LiteralDataNode::PlainString(PlainStringDataNode { value }),
                    ));
                  }
                  Token::FunctionCallCloseParenthesis => {
                    break;
                  }
                  token => todo!("Unexpected token: {:?}", token),
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

#[derive(Debug)]
pub struct Ast {
  imports: Vec<ImportNode>,
  items: Vec<ItemNode>,
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
struct ImportNode {
  import_assignment: ImportAssignmentNode,
  import_source_token: String,
}

#[derive(Debug)]
enum ImportAssignmentNode {
  Destructured(DestructureAssignmentNode),
}

#[derive(Debug)]
struct DestructureAssignmentNode {
  fields: Vec<DestructureAssignmentFieldNode>,
}

#[derive(Debug)]
struct DestructureAssignmentFieldNode {
  field_token: String,
  alias: Option<DestructureAssignmentFieldAliasNode>,
}

#[derive(Debug)]
struct DestructureAssignmentFieldAliasNode {
  name_token: String,
}

#[derive(Debug)]
enum ItemNode {
  Function(FunctionNode),
}

#[derive(Debug)]
struct FunctionNode {
  signature: FunctionSignatureNode,
  body: FunctionBodyNode,
}

#[derive(Debug)]
struct FunctionSignatureNode {
  is_public: bool,
  name_token: String,
  params: Vec<FunctionParamNode>,
  return_type: Option<ReturnTypeNode>,
}

#[derive(Debug)]
struct FunctionParamNode {}

#[derive(Debug)]
struct ReturnTypeNode {}

#[derive(Debug)]
struct FunctionBodyNode {
  statements: Vec<StatementNode>,
}

#[derive(Debug)]
enum StatementNode {
  Expression(ExpressionNode),
}

#[derive(Debug)]
enum ExpressionNode {
  Call(ExpressionCallNode),
}

#[derive(Debug)]
struct ExpressionCallNode {
  subject: ExpressionCallPathNode,
  args: Vec<ExpressionCallArgNode>,
}

#[derive(Debug)]
struct ExpressionCallPathNode {
  segments: Vec<ExpressionCallPathSegmentNode>,
}

#[derive(Debug)]
enum ExpressionCallPathSegmentNode {
  TypeIdentity(String),
  FunctionIdentity(String),
}

#[derive(Debug)]
enum ExpressionCallArgNode {
  Literal(LiteralDataNode),
}

#[derive(Debug)]
enum LiteralDataNode {
  PlainString(PlainStringDataNode),
}

#[derive(Debug)]
struct PlainStringDataNode {
  value: String,
}
