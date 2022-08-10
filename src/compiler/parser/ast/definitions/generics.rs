use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DefGenerics {
    pub generics: DefGeneric,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefGeneric {
    pub name: String,
    pub constraints: Option<DefGenericConstraints>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefGenericConstraints {
    pub types: Vec<Type>,
}

