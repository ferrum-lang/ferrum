use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ItemTrait {
    pub is_public: bool,
    pub name: String,
    pub generics: Generics,
    pub items: Vec<TraitItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TraitItem {
    Const(TraitItemConst),
    Method(TraitItemMethod),
    Type(TraitItemType),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TraitItemConst {
    pub name: String,
    pub typ: Type,
    pub default: Option<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TraitItemMethod {
    pub signature: MethodSignature,
    pub default: Option<Block>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TraitItemType {
    pub name: String,
    pub generics: Generics,
    pub bounds: Vec<TypeParamBound>,
    pub default: Option<Type>,
}

