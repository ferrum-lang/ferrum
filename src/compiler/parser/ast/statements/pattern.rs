use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Literal(PatternLiteral),
    Struct(PatternStruct),
    TupleStruct(PatternTupleStruct),
    Tuple(PatternTuple),
    List(PatternList),
    Identity(PatternIdentity),
    Wild(PatternWild),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternLiteral {}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternStruct {}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternTupleStruct {}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternTuple {}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternList {}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternIdentity {}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternWild {}


