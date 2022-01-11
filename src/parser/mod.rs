use super::types::*;

type ParserFn = Vec<fn(&mut String, &mut Vec<Definition>) -> bool>;

pub fn parse_definitions_from_lang(contents: String) -> Result<Vec<Definition>, String> {
  let mut definitions: Vec<Definition> = Vec::new();
  let mut unparsed = String::from(contents.trim());

  let parsers: ParserFn = vec![parse_import, parse_function];

  while !unparsed.is_empty() {
    if parsers
      .iter()
      .any(|parser| parser(&mut unparsed, &mut definitions))
    {
      continue;
    }

    unparsed = String::from(&unparsed[1..]);
  }

  return Ok(definitions);
}

fn parse_import(unparsed: &mut String, definitions: &mut Vec<Definition>) -> bool {
  if unparsed.starts_with("import {") {
    let unparsed_import = &unparsed["import {".len()..];

    let end_of_parts_import = unparsed_import.find('}').expect("'}' not found in import");

    let parts = unparsed_import[..end_of_parts_import]
      .split(",")
      .into_iter()
      .map(|i| String::from(i.trim()))
      .filter(|i| !i.is_empty())
      .collect::<Vec<String>>();

    let semicolon = unparsed_import
      .find(';')
      .expect("';' not found after import");

    let source =
      String::from(&unparsed_import[end_of_parts_import + " from \"".len() + 1..semicolon - 1]);

    let definition = Definition::Import {
      source,
      imported_data: ImportedData::Destructured { parts },
    };

    definitions.push(definition);

    let new_unparsed = String::from((&unparsed[semicolon + "import {".len() + 1..]).trim());

    unparsed.replace_range(.., new_unparsed.as_str());

    return true;
  }

  return false;
}

fn parse_function(unparsed: &mut String, definitions: &mut Vec<Definition>) -> bool {
  let is_public = unparsed.starts_with("public ");
  let offset = if is_public { "public ".len() } else { 0 };

  let unparsed_function = String::from((&unparsed[offset..]).trim());

  if unparsed_function.starts_with("function ") {
    let start_of_params = unparsed_function
      .find('(')
      .expect("'(' not found after function name");

    let name = String::from((&unparsed_function["function ".len()..start_of_params]).trim());

    let start_of_expressions = unparsed_function
      .find('{')
      .expect("'{' not found after function params");

    let end_of_expressions = unparsed_function
      .find('}')
      .expect("'}' not found after function params");

    let expression_str = &unparsed_function[start_of_expressions + 1..end_of_expressions];

    let mut expressions_vec = vec![];

    {
      let index_of_type_access = expression_str
        .find("::")
        .expect("Temporary issue: Expected '::' used to access a type in function's expression");

      let type_name = String::from((&expression_str[..index_of_type_access]).trim());

      let start_of_args = expression_str
        .find('(')
        .expect("Temporary issue: Expected '(' used to call a type's function");

      let function_name =
        String::from((&expression_str[index_of_type_access + 2..start_of_args]).trim());

      let end_of_args = expression_str
        .find(')')
        .expect("Temporary issue: Expected ')' used to finish calling a type's function");

      let arg = String::from((&expression_str[start_of_args + 2..end_of_args - 1]).trim());

      expressions_vec.push(FullFunctionExpression::Action {
        action: ExpressionAction::TypeAccess {
          name: type_name,
          access: TypeAccess::FunctionCall(ActionFunctionCall {
            name: function_name,
            args: FunctionArgs(vec![ExpressionAction::Data(DataType::PlainString(arg))]),
          }),
        },
      });
    }

    let definition = Definition::Function {
      visibility: if is_public {
        Visibility::Public
      } else {
        Visibility::Private
      },
      name,
      params: FunctionParams(vec![]),
      return_type: FunctionReturnType(None),
      expressions: FullFunctionExpressions(expressions_vec),
    };

    definitions.push(definition);

    unparsed.replace_range(..unparsed.find('}').unwrap() + 1, "");

    return true;
  }

  return false;
}

// return Ok(vec![
//   Definition::Import {
//     source: "std".to_string(),
//     imported_data: ImportedData::Destructured {
//       parts: vec!["Console".to_string()],
//     },
//   },
//   // Definition::Import {
//   //   source: "std".to_string(),
//   //   imported_data: ImportedData::Whole {
//   //     name: "custom".to_string(),
//   //   },
//   // },
//   // Definition::Import {
//   //   source: "std".to_string(),
//   //   imported_data: ImportedData::All,
//   // },
//   Definition::Function {
//     visibility: Visibility::Public,
//     name: "main".to_string(),
//     params: FunctionParams(vec![]),
//     return_type: FunctionReturnType(None),
//     expressions: FullFunctionExpressions(vec![FullFunctionExpression::Action {
//       action: ExpressionAction::TypeAccess {
//         name: "Console".to_string(),
//         access: TypeAccess::FunctionCall(ActionFunctionCall {
//           name: "write_line".to_string(),
//           args: FunctionArgs(vec![ExpressionAction::Data(DataType::PlainString(
//             "Hello world".to_string(),
//           ))]),
//         }),
//       },
//     }]),
//   },
// ]);
// }
