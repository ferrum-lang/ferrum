use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    pub reciever: Option<Type>,
    pub name: String,
    pub args: Vec<FunctionCallArg>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCallArg {
    pub name: Option<String>,
    pub value: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodCall {
    pub reciever: Box<Expression>,
    pub name: String,
    pub args: Vec<FunctionCallArg>,
}

