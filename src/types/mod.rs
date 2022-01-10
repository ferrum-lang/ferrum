#[derive(Debug)]
pub enum Definition {
  Import {
    source: String,
    imported_data: ImportedData,
  },
  Function {
    visibility: Visibility,
    name: String,
    params: FunctionParams,
    return_type: FunctionReturnType,
    expressions: FullFunctionExpressions,
  },
  Contract {},
  Structure {},
}

#[derive(Debug)]
pub enum ImportedData {
  Whole { name: String },
  Destructured { parts: Vec<String> },
  All,
}

#[derive(Debug)]
pub enum Visibility {
  Private,
  Public,
}

#[derive(Debug)]
pub struct FunctionParams(pub Vec<FunctionParam>);

#[derive(Debug)]
pub struct FunctionParam {}

#[derive(Debug)]
pub struct FunctionReturnType(pub Option<String>);

#[derive(Debug)]
pub struct FullFunctionExpressions(pub Vec<FullFunctionExpression>);

#[derive(Debug)]
pub enum FullFunctionExpression {
  Assignment {
    assignment: ExpressionAssignment,
    action: ExpressionAction,
  },
  Action {
    action: ExpressionAction,
  },
  Loop {},
}

#[derive(Debug)]
pub enum ExpressionAction {
  TypeAccess {
    name: String,
    access: TypeAccess,
  },
  InstanceAccess {
    name: String,
    access_type: InstanceAccess,
    mutability: Mutability,
    ownership: Ownership,
  },
  Data(DataType),
  FunctionCall(ActionFunctionCall),
  Condition {},
  ConditionControl {},
}

#[derive(Debug)]
pub enum TypeAccess {
  FunctionCall(ActionFunctionCall),
}

#[derive(Debug)]
pub enum InstanceAccess {}

#[derive(Debug)]
pub enum DataType {
  PlainString(String),
  FormattedString(String),
}

#[derive(Debug)]
pub struct ActionFunctionCall {
  pub name: String,
  pub args: FunctionArgs,
}

#[derive(Debug)]
pub struct FunctionArgs(pub Vec<ExpressionAction>);

#[derive(Debug)]
pub enum ExpressionAssignment {
  Declaration { variable_type: VariableType },
  Reassignment { name: String },
}

#[derive(Debug)]
pub enum VariableType {
  Const,
  Let,
}

#[derive(Debug)]
pub enum Mutability {
  Immutable,
  Mutable,
}

#[derive(Debug)]
pub enum Ownership {
  Owned,
  Borrowed,
}
