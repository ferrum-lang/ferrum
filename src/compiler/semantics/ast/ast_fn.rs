use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ItemFn {
    pub is_public: bool,
    pub signature: FnSignature,
    pub block: Box<Block>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnSignature {
    pub is_const: bool,
    pub is_async: bool,
    pub name: String,
    pub generics: Generics,
    pub params: Vec<FnParam>,
    pub return_type: ReturnType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FnParam {
    Self_(Self_),
    Typed(PatType),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Self_ {
    pub is_mutable: bool,
    pub reference: Option<Reference>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Reference {
    pub lifetime: Option<Lifetime>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Lifetime {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnType {
    Default,
    Type(Box<Type>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

