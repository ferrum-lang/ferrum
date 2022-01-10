use super::types::*;
use std::fmt::{Display, Error, Formatter};

pub fn build_rust_from_definitions(definitions: Vec<Definition>) -> Result<String, String> {
  let mut contents = "mod lang_prelude;\nuse lang_prelude::*;\n".to_string();

  for definition in definitions {
    contents.push_str(&*format!("\n{}", definition));
  }

  return Ok(contents);
}

impl Display for Definition {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    match self {
      Definition::Import {
        source,
        imported_data,
      } => {
        let source = {
          let mut tmp = String::from("lang_");
          tmp.push_str(source);
          tmp
        };
        match imported_data {
          ImportedData::Destructured { parts } => write!(
            formatter,
            "mod {source};\nuse {source}::{{ {parts} }};\n",
            source = source,
            parts = parts.join(", "),
          ),
          ImportedData::Whole { name } => write!(
            formatter,
            "mod {source};\nuse {source} as {name};\n",
            source = source,
            name = name
          ),
          ImportedData::All => write!(
            formatter,
            "mod {source};\nuse {source}::*;\n",
            source = source
          ),
        }
      }
      Definition::Function {
        visibility,
        name,
        params,
        return_type,
        expressions,
      } => {
        write!(
          formatter,
          "{}fn {}({}){} {{\n{}\n}}\n",
          visibility, name, params, return_type, expressions
        )
      }
      Definition::Structure {} => todo!("TODO: {:?}", self),
      Definition::Contract {} => todo!("TODO: {:?}", self),
    }
  }
}

impl Display for Visibility {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    match self {
      Visibility::Private => write!(formatter, ""),
      Visibility::Public => write!(formatter, "pub "),
    }
  }
}

impl Display for FunctionParams {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    write!(
      formatter,
      "{}",
      self
        .0
        .iter()
        .map(|param| format!("{}", param))
        .collect::<Vec<String>>()
        .join(", ")
    )
  }
}

impl Display for FunctionParam {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    todo!("{:?}", self)
  }
}

impl Display for FunctionReturnType {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    match &self.0 {
      Some(return_type) => write!(formatter, " -> {}", return_type),
      None => write!(formatter, ""),
    }
  }
}

impl Display for FullFunctionExpressions {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    write!(
      formatter,
      "{}",
      self
        .0
        .iter()
        .map(|expression| format!("{}", expression))
        .collect::<Vec<String>>()
        .join(";\n")
    )
  }
}

impl Display for FullFunctionExpression {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    match self {
      FullFunctionExpression::Action { action } => write!(formatter, "{};\n", action),
      _ => todo!("{:?}", self),
    }
  }
}

impl Display for ExpressionAction {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    match self {
      ExpressionAction::TypeAccess { name, access } => {
        write!(formatter, "{}::{}", name, access)
      }
      ExpressionAction::Data(data) => write!(formatter, "{}", data),
      _ => todo!("{:?}", self),
    }
  }
}

impl Display for TypeAccess {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    match self {
      TypeAccess::FunctionCall(ActionFunctionCall { name, args }) => {
        write!(formatter, "{}({})", name, args)
      }
      _ => todo!("{:?}", self),
    }
  }
}

impl Display for FunctionArgs {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    write!(
      formatter,
      "{}",
      self
        .0
        .iter()
        .map(|arg| format!("{}", arg))
        .collect::<Vec<String>>()
        .join(", ")
    )
  }
}

impl Display for DataType {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    match self {
      DataType::PlainString(plain_string) => {
        write!(formatter, "LangString::from_slice(\"{}\")", plain_string)
      }
      _ => todo!("{:?}", self),
    }
  }
}
