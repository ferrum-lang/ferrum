use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Construction {
    pub r#type: Type,
    pub fields: Vec<ConstructionField>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstructionField {
    Assign(ConstructionFieldAssign),
    Spread(ConstructionFieldSpread),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionFieldAssign {
    pub name: String,
    pub value: Option<Box<Expression>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionFieldSpread {
    pub value: Box<Expression>,
}

