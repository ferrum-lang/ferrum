#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
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
  FunctionParamsParamTypeColon,
  FunctionParamsParamTypeMutable,
  FunctionParamsParamTypeBorrowed,
  FunctionParamsParamTypeShared,
  FunctionParamsParamTypeName(String),
  FunctionParamsComma,
  FunctionParamsCloseParenthesis,
  FunctionReturnTypeColon,
  FunctionReturnTypeMutable,
  FunctionReturnTypeBorrowed,
  FunctionReturnTypeShared,
  FunctionReturnTypeName(String),
  FunctionExpressionsOpenBrace,
  FunctionExpressionsCloseBrace,
  TypeAccessDoubleSemicolon,
  TypeAccessName(String),
  InstanceBorrow,
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
  VariableTypeColon,
  TypeMutable,
  TypeBorrowed,
  TypeShared,
  TypeName(String),
  TypeGenericOpen,
  TypeGenericComma,
  TypeGenericClose,
  TypeOptional,
  Assignment,
  True,
  False,
  Char(String),
  Int(String),
  Float(String),
  TupleTypeStart,
  TupleTypeSemicolon,
  TupleTypeLength(usize),
  TupleTypeComma,
  TupleTypeEnd,
  TupleStart,
  TupleComma,
  TupleSemicolon,
  TupleLength(usize),
  TupleEnd,
  ListOpen,
  ListComma,
  ListClose,
  Plus,
  Minus,
  Multiply,
  Divide,
  Exponent,
  Range,
  ClosureParamsOpen,
  ClosureParamsComma,
  ClosureParamsClose,
  ClosureArrow,
  TernaryQuestion,
  TernaryOr,
  CastQuestion,
  NullCoalesce,
  // TODO: continue adding ...
}
