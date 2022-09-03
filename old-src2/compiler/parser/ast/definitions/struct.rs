use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DefStruct {
    pub name: String,
    pub generics: Option<DefGenerics>,
    pub fields: Vec<DefStructField>,
    pub is_public: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefStructField {
    pub name: String,
    pub r#type: Type,
    pub default: Option<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefTupleStruct {
    pub name: String,
    pub generics: Option<DefGenerics>,
    pub tuple_type: TypeTuple,
    pub is_public: bool,
}
