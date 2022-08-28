use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Reference {
    Static(ReferenceStatic),
    Instance(ReferenceInstance),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceStatic {
    pub receiver: Option<Box<ReferenceStatic>>,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceInstance {
    pub receiver: Option<Box<Expression>>,
    pub name: String,
}
