use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DefStruct {
    pub name: String,
    pub generics: DefGenerics,
    pub fields: Vec<DefStructField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefStructField {
    pub name: String,
    pub r#type: Type,
    pub default: Option<Box<Expression>>,
}

