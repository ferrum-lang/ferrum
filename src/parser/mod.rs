use super::types::*;

pub fn parse_definitions_from_lang(contents: String) -> Result<Vec<Definition>, String> {
  // let mut tokens: Vec<Token> = Vec::new();

  // let mut is_in_string = false;
  // let mut next_escaped;

  // let chars_iter = contents.chars();

  // for c in chars_iter {
  //     if !is_in_string {
  //         if c.is_whitespace() {
  //             println!("Whitespace.");
  //             continue;
  //         }

  //         if c == '"' {
  //             is_in_string = true;
  //             println!("STARTING STRING");
  //         }
  //     } else {
  //         if c == '\\' {
  //             next_escaped = true;
  //         } else {
  //             next_escaped = false;
  //         }

  //         if c == '"' && !next_escaped {
  //             is_in_string = false;
  //             println!("DONE STRING");
  //         }
  //     }

  //     println!("Char: \"{}\"", c);
  // }

  return Ok(vec![
    Definition::Import {
      source: "std".to_string(),
      imported_data: ImportedData::Destructured {
        parts: vec!["Console".to_string()],
      },
    },
    Definition::Function {
      visibility: Visibility::Public,
      name: "main".to_string(),
      params: FunctionParams(vec![]),
      return_type: FunctionReturnType(None),
      expressions: FullFunctionExpressions(vec![FullFunctionExpression::Action {
        action: ExpressionAction::TypeAccess {
          name: "Console".to_string(),
          access: TypeAccess::FunctionCall(ActionFunctionCall {
            name: "write_line".to_string(),
            args: FunctionArgs(vec![ExpressionAction::Data(DataType::PlainString(
              "Hello world".to_string(),
            ))]),
          }),
        },
      }]),
    },
  ]);
}
