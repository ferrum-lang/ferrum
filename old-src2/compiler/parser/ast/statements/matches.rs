use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Matches {
    pub value: Box<Expression>,
    pub pattern: Pattern,
}

