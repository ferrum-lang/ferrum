#[derive(Debug, PartialEq, Eq)]
pub enum Token {
  Whitespace,
  Semicolon,
  Mutable,
  ReferenceAmpersand,
  DestructureOpenBrace,
  DestructureField(String),
  DestructureAliasColon,
  DestructureAliasName(String),
  DestructureComma,
  DestructureCloseBrace,
  Import,
  ImportFrom,
  ImportSource(String),
  Public,
  Function,
  FunctionName(String),
  FunctionParamsOpenParenthesis,
  FunctionParamsParamName(String),
  FunctionParamsComma,
  FunctionParamsCloseParenthesis,
  FunctionExpressionsOpenBrace,
  FunctionExpressionsCloseBrace,
  TypeAccessDoubleSemicolon,
  TypeAccessName(String),
  InstanceReferenceName(String),
  InstanceAccessName(String),
  FunctionCallName(String),
  TypeAccessColons,
  InstanceAccessPeriod,
  FunctionCallOpenParenthesis,
  FunctionCallComma,
  FunctionCallCloseParenthesis,
  PlainString(String),
  TemplateStringStart(String),
  TemplateStringMiddle(String),
  TemplateStringEnd(String),
  TemplateStringTemplateOpenBrace,
  TemplateStringTemplateCloseBrace,
  Let,
  Const,
  VariableName(String),
  Assignment,
  Char(String),
  Int(String),
  Float(String),
  // TODO: continue adding ...
}
