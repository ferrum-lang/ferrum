use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ItemImpl {
    pub generics: Generics,
    pub trait_: Option<Path>,
    pub self_type: Box<Type>,
    pub items: Vec<ImplItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImplItem {
    Const(ImplItemConst),
    Method(ImplItemMethod),
    Type(ImplItemType),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImplItemConst {
    pub is_public: bool,
    pub name: String,
    pub typ: Type,
    pub expr: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImplItemMethod {
    pub is_public: bool,
    pub signature: MethodSignature,
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodSignature {
    pub is_const: bool,
    pub is_async: bool,
    pub name: String,
    pub generics: Generics,
    pub inputs: Vec<FnParam>,
    pub output: ReturnType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImplItemType {
    pub is_public: bool,
    pub name: String,
    pub generics: Generics,
    pub typ: Type,
}

