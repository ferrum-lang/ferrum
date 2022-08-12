use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Number(LiteralNumber),
    String(LiteralString),
    Char(char),
    Bool(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralNumber {
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    Uint(usize),
    // BigUint,

    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Int(isize),
    // BigInt,

    Bit(bool),
    Byte(u8),

    Float32(f32),
    Float64(f64),
    Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralString {
    Plain(String),
    Template(TemplateString),
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

