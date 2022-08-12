use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DefClass {
    pub static_consts: Vec<StaticConst>,
    pub functions: Vec<DefFn>,
    pub self_state: DefClassSelfState,
    pub methods: Vec<DefClassMethod>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefClassSelfState {
    pub fields: Vec<DefClassSelfStateField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefClassSelfStateField {
    pub is_public: bool,
    pub name: String,
    pub r#type: Type,
    pub default: Option<Box<Expression>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefClassMethod {
    pub signature: DefClassMethodSignature,
    pub r#impl: DefFnImpl,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefClassMethodSignature {
    pub is_public: bool,
    pub requires_mutable: bool,
    pub name: String,
    pub generics: Option<DefGenerics>,
    pub params: Vec<DefFnParam>,
    pub return_type: Option<Type>,
}

