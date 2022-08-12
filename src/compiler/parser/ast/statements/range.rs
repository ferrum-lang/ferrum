use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    from: LiteralNumber,
    to: LiteralNumber,
    inclusive: bool,
}


