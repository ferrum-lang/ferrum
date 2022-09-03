use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
    Number(LiteralNumber),
    Char(LiteralChar),
    PlainString(LiteralString),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiteralNumber {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiteralChar {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiteralString {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateString {
    pub start: String,
    pub parts: Vec<TemplateStringPart>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateStringPart {
    pub expression: Box<Expression>,
    pub post_string: String,
}
