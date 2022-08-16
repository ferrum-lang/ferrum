use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    pub from: Box<Expression>,
    pub to: Box<Expression>,
    pub inclusive: bool,
}

