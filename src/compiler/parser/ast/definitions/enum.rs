use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DefEnum {
    pub values: Vec<DefEnumField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefEnumField {
    pub name: String,
    pub data: Option<DefEnumFieldData>,
    pub const_value: Option<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DefEnumFieldData {
    Struct(DefEnumFieldDataStruct),
    Tuple(DefEnumFieldDataTuple),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefEnumFieldDataStruct {
    pub fields: Vec<DefStructField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefEnumFieldDataTuple {
    pub types: Vec<DefEnumFieldDataTupleType>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefEnumFieldDataTupleType {
    pub r#type: Type,
    pub default: Option<Box<Expression>>,
}
