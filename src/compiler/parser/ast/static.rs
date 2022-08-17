use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct StaticConst {
    pub name: String,
    pub value: Expression,
    pub r#type: Option<Type>,
    pub is_public: bool,
}

