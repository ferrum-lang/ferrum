use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum List {
    Explicit(ListExplicit),
    FnFor(ListFnFor),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListExplicit {
    pub values: Vec<Box<Expression>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListFnFor {
    pub item: AssignmentTarget,
    pub r#in: Box<Expression>,
    pub expression: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListIndexedItem {
    pub receiver: Box<Expression>,
    pub index: Box<Expression>,
}

