use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Tuple {
    Explicit(TupleExplicit),
    Repeated(TupleRepeated),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TupleExplicit {
    pub values: Vec<Box<Expression>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TupleRepeated {
    pub value: Box<Expression>,
    pub count: LiteralNumber,
}

