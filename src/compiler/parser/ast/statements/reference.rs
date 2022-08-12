use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Reference {
    Static(ReferenceStatic),
    Instance(ReferenceInstance),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceStatic {
    pub reciever: Type,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceInstance {
    pub receiver: Option<Box<Expression>>,
    pub name: String,
}
