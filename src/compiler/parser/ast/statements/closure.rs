use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Closure {
    pub signature: DefFnSignature,
    pub r#impl: Box<DefFnImpl>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClosureSignature {
    pub generics: Option<DefGenerics>,
    pub params: Vec<DefFnParam>,
    pub return_type: Option<Type>,
}

