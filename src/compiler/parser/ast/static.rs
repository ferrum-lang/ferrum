use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct StaticConst {
    pub name: String,
    pub value: Box<Expression>,
    pub r#type: Option<Type>,
}
