use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub is_const: Option<bool>,
    pub explicit_type: Option<Type>,
    pub target: AssignmentTarget,
    pub expression: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentTarget {
    Direct(String),
    DestructureObject(AssignTrgtDestructObject),
    DestructureTuple(AssignTrgtDestructTuple),
    DestructureList(AssignTrgtDestructList),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignTrgtDestructObject {
    pub items: Vec<AssignTrgtDestructObjectItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssignTrgtDestructObjectItem {
    Field(AssignTrgtDestructObjectField),
    SpreadField(AssignTrgtDestructObjectSpreadField),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignTrgtDestructObjectField {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignTrgtDestructObjectSpreadField {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignTrgtDestructTuple {
    pub items: Vec<AssignTrgtDestructTupleItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssignTrgtDestructTupleItem {
    Field(AssignTrgtDestructTupleField),
    SpreadField(AssignTrgtDestructTupleSpreadField),
    Spread,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignTrgtDestructTupleField {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignTrgtDestructTupleSpreadField {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignTrgtDestructList {
    pub items: Vec<AssignTrgtDestructListItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssignTrgtDestructListItem {
    Field(AssignTrgtDestructListField),
    SpreadField(AssignTrgtDestructListSpreadField),
    Spread,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignTrgtDestructListField {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignTrgtDestructListSpreadField {
    pub name: String,
}

