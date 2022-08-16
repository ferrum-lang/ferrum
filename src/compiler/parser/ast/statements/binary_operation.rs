use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: BinaryOperator,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Mod,
    Pow,

    PlusEquals,
    MinusEquals,
    MultiplyEquals,
    DivideEquals,
    ModEquals,
    PowEquals,

    And,
    Or,
    
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,

    NullCoalesce,
}

