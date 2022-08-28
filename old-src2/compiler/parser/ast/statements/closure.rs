use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Closure {
    pub signature: ClosureSignature,
    pub r#impl: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClosureSignature {
    pub params: Vec<ClosureParam>,
    pub return_type: Option<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClosureParam {
    pub name: String,
    pub r#type: Option<Type>,
}

