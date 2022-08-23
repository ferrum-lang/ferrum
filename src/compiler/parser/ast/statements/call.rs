use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    pub receiver: Option<ReferenceStatic>,
    pub name: String,
    pub args: Vec<FunctionCallArg>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodCall {
    pub receiver: Box<Expression>,
    pub name: String,
    pub args: Vec<FunctionCallArg>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCallArg {
    pub name: Option<String>,
    pub value: Box<Expression>,
}

