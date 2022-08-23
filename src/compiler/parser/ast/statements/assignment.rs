use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    pub local_var: Option<AssignmentLocalVar>,
    pub explicit_type: Option<Type>,
    pub target: AssignmentTarget,
    pub expression: Option<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentLocalVar {
    Const,
    Let,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentTarget {
    Direct(String),
    Reference(ReferenceInstance),
    DestructureObject(AssignTrgtDestructObject),
    DestructureTuple(AssignTrgtDestructTuple),
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
    pub value: Box<AssignmentTarget>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignTrgtDestructTupleSpreadField {
    pub name: String,
}

