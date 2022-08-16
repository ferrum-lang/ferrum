use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    BuiltIn(TypeBuiltIn),

    Optional(Option<Box<Type>>),
    Result(Option<Box<Type>>),

    Tuple(TypeTuple),
    List(TypeList),

    Custom(TypeCustom),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeBuiltIn {
    Bool,

    Bit,
    Byte,

    Uint,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    BigUint,

    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    BigInt,

    Float,
    Float32,
    Float64,

    Char,

    String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeTuple {
    Repeated(TypeTupleRepeated),
    Explicit(TypeTupleExplicit),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeTupleRepeated {
    pub r#type: Box<Type>,
    pub count: LiteralNumber,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeTupleExplicit {
    pub types: Vec<Box<Type>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeList {
    pub r#type: Box<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeCustom {
    pub name: String,
    pub receiver: Option<Box<TypeCustom>>,
}

