use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    pub from: LiteralNumber,
    pub to: LiteralNumber,
    pub inclusive: bool,
}

