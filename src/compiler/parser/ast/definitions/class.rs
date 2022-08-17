use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DefClass {
    pub is_public: bool,
    pub name: String,
    pub generics: Option<DefGenerics>,
    pub static_consts: Vec<StaticConst>,
    pub functions: Vec<DefFn>,
    pub self_state: Option<DefClassSelfState>,
    pub construct: Option<DefClassConstruct>,
    pub methods: Vec<DefClassMethod>,
    pub impls: Vec<DefClassImpl>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefClassSelfState {
    pub fields: Vec<DefClassSelfStateField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefClassConstruct {
    pub is_public: bool,
    pub details: DefClassConstructDetails,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DefClassConstructDetails {
    Default,
    WithParamsOnly(DefClassConstructWithParams),
    Full(DefClassConstructFull),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefClassConstructWithParams {
    pub params: Vec<DefClassConstructParam>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefClassConstructFull {
    pub params: Vec<DefClassConstructParam>,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DefClassConstructParam {
    Direct(DefClassConstructParamDirect),
    Spread,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefClassConstructParamDirect {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefClassSelfStateField {
    pub is_public: bool,
    pub is_const: bool,
    pub is_mut: bool,
    pub name: String,
    pub r#type: Type,
    pub default: Option<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefClassMethod {
    pub signature: DefClassMethodSignature,
    pub r#impl: DefFnImpl,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefClassMethodSignature {
    pub is_public: bool,
    pub is_mut: bool,
    pub name: String,
    pub generics: Option<DefGenerics>,
    pub params: Vec<DefFnParam>,
    pub return_type: Option<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefClassImpl {
    pub name: String,
    pub methods: Vec<DefClassMethod>,
}

