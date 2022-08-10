use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DefGenerics {
    generics: DefGeneric,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefGeneric {
    name: String,
    constraints: Option<DefGenericConstraints>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefGenericConstraints {
    types: Vec<Type>,
}

