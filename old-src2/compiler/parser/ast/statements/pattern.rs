use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Literal(PatternLiteral),
    Struct(PatternStruct),
    Object(PatternObject),
    TupleStruct(PatternTupleStruct),
    Tuple(PatternTuple),
    List(PatternList),
    Identity(PatternIdentity),
    Wild(PatternWild),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PatternLiteral {
    Bool(bool),
    Char(String),
    Number(String),
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternStruct {
    pub name: String,
    pub fields: Vec<PatternStructField>,
    pub receiver: Option<PatternIdentity>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternObject {
    pub fields: Vec<PatternStructField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternStructField {
    pub name: String,
    pub pattern: Option<Box<Pattern>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternTupleStruct {
    pub name: String,
    pub args: Vec<Box<Pattern>>,
    pub receiver: Option<PatternIdentity>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternTuple {
    pub values: Vec<Box<Pattern>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternList {
    pub values: Vec<Box<Pattern>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternIdentity {
    pub name: String,
    pub receiver: Option<Box<PatternIdentity>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternWild {}


