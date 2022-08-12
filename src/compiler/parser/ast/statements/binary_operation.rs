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

    And,
    Or,
    
    Equals,
    NotEquals,
    GreaterThan,
    GreatherThanOrEquals,
    LessThan,
    LessThanOrEquals,
}

