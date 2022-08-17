use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DefInterface {
    pub is_public: bool,
    pub name: String,
    pub generics: Option<DefGenerics>,
    pub methods: Vec<DefInterfaceMethod>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefInterfaceMethod {
    pub is_mut: bool,
    pub name: String,
    pub generics: Option<DefGenerics>,
    pub params: Vec<DefFnParam>,
    pub return_type: Option<Type>,
}

