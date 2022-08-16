use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DefInterface {
    pub methods: Vec<DefInterfaceMethod>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefInterfaceMethod {
    pub requires_mutable: bool,
    pub name: String,
    pub generics: Option<DefGenerics>,
    pub params: Vec<DefFnParam>,
    pub return_type: Option<Type>,
}
