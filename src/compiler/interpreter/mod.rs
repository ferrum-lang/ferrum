use super::{ast::*, Error};

const IMPORT_PREFIX: &'static str = "lang_";

const INCLUDE_PRELUDE: &'static str = "mod lang_prelude;\nuse lang_prelude::*;\n";

pub fn generate_rust(mut ast: Ast) -> Result<String, Error> {
  let mut rs = String::from(INCLUDE_PRELUDE);

  ast.imports.reverse();
  while let Some(import) = ast.imports.pop() {
    rs.push_str(&generate_import(&mut ast, import)?);
  }

  ast.items.reverse();
  while let Some(item) = ast.items.pop() {
    match item {
      ItemNode::Function(function) => {
        rs.push_str(&generate_function(&mut ast, function)?);
      }
    }
  }

  return Ok(rs);
}

pub fn generate_import(_ast: &mut Ast, import: ImportNode) -> Result<String, Error> {
  let mut import_rs = format!(
    "mod {}{};\nuse {}{}::",
    IMPORT_PREFIX, import.source_token, IMPORT_PREFIX, import.source_token
  );

  match import.assignment {
    ImportAssignmentNode::Destructured(mut destructure) => {
      import_rs.push_str("{\n");

      destructure.fields.reverse();
      while let Some(field) = destructure.fields.pop() {
        if let Some(alias) = field.alias {
          import_rs.push_str(&format!("{}: {},\n", field.field_token, alias.name_token));
        } else {
          import_rs.push_str(&format!("{},\n", field.field_token));
        }
      }

      import_rs.push_str("};\n");
    }
    node => todo!("Unexpected node: {:?}", node),
  }

  return Ok(import_rs);
}

pub fn generate_function(mut ast: &mut Ast, mut function: FunctionNode) -> Result<String, Error> {
  let mut function_rs = String::new();

  if function.signature.is_public {
    function_rs.push_str("pub ");
  }

  function_rs.push_str(&format!("fn {}(\n", function.signature.name_token));
  function_rs.push_str(")\n");
  function_rs.push_str("{\n");

  function.body.statements.reverse();
  while let Some(statement) = function.body.statements.pop() {
    function_rs.push_str(&generate_statement(&mut ast, statement)?);
  }

  function_rs.push_str("}\n");

  return Ok(function_rs);
}

pub fn generate_statement(mut ast: &mut Ast, statement: StatementNode) -> Result<String, Error> {
  let mut statement_rs;

  match statement {
    StatementNode::Expression(expression) => {
      statement_rs = generate_expression(&mut ast, expression)?
    }
    node => todo!("Unexpected node: {:?}", node),
  }

  return Ok(statement_rs);
}

pub fn generate_expression(mut ast: &mut Ast, expression: ExpressionNode) -> Result<String, Error> {
  let mut expression_rs = String::new();

  match expression {
    ExpressionNode::Call(mut call) => {
      call.subject.segments.reverse();
      while let Some(segment) = call.subject.segments.pop() {
        match segment {
          ExpressionCallPathSegmentNode::TypeIdentity(name) => {
            expression_rs.push_str(&format!("{}::", name));
          }
          ExpressionCallPathSegmentNode::FunctionIdentity(name) => {
            expression_rs.push_str(&name);
          }
          node => todo!("Unexpected node: {:?}", node),
        }
      }

      expression_rs.push_str("(");

      call.args.reverse();
      while let Some(arg) = call.args.pop() {
        expression_rs.push_str(&generate_expression(&mut ast, arg)?);

        if call.args.len() > 0 {
          expression_rs.push_str(", ");
        }
      }

      expression_rs.push_str(");\n");
    }
    ExpressionNode::Literal(literal) => match literal {
      LiteralDataNode::PlainString(value) => {
        expression_rs.push_str(&format!("LangString::from_slice(\"{}\")", value));
      }
      LiteralDataNode::Integer(value) => {
        expression_rs.push_str(&format!("{}", value));
      }
      LiteralDataNode::TemplateString(mut template_string) => {
        let mut string = template_string.start_token;

        template_string.middle_tokens.reverse();
        while let Some(middle) = template_string.middle_tokens.pop() {
          string.push_str("{}");
          string.push_str(&middle);
        }

        string.push_str("{}");
        string.push_str(&template_string.end_token);

        let mut args = String::new();

        template_string.expressions.reverse();
        while let Some(expression) = template_string.expressions.pop() {
          args.push_str(&generate_expression(&mut ast, expression)?);

          if template_string.expressions.len() > 0 {
            args.push_str(", ");
          }
        }

        expression_rs.push_str(&format!(
          "LangString::from_owned(format!(\"{}\", {}))",
          string, args
        ));
      }
      node => todo!("Unexpected node: {:?}", node),
    },
    node => todo!("Unexpected node: {:?}", node),
  }

  return Ok(expression_rs);
}
