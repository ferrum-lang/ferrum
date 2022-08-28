use std::collections::HashMap;

pub struct Meta {
    pub functions: HashMap<String, MetaFn>,
    pub structs: HashMap<String, MetaStruct>,
    pub classes: HashMap<String, MetaClass>,
    pub interfaces: HashMap<String, MetaInterface>,
    pub enums: HashMap<String, MetaEnum>,
}

pub struct MetaFn {
    pub name: String,
    pub params: Vec<MetaFnParam>,
    pub return_type: Option<MetaReturnType>,
}

pub struct MetaFnParam {
    pub name: String,
    pub typ: MetaParamType,
    pub is_mut: bool,
    pub requires_ref: Option<bool>,
}

pub struct MetaType {
    pub is_primitive: bool,
}

pub struct MetaStruct {
    pub name: String,
    pub fields: HashMap<String, MetaStructField>,
}

pub struct MetaStructField {
    pub name: String,
    // pub is_mut: bool,
}

pub struct MetaClass {
    pub name: String,
    pub fields: HashMap<String, MetaClassField>,
    pub methods: HashMap<String, MetaMethod>,
    pub impls: HashMap<String, MetaImpl>,
}

pub struct MetaClassField {
    pub name: String,
    pub is_reassign: bool,
    pub is_mut: bool,
}

pub struct MetaImpl {
    pub name: String,
    pub methods: HashMap<String, MetaMethod>,
}

pub struct MetaInterface {
    pub name: String,
    pub methods: HashMap<String, MetaMethod>,
}

pub struct MetaMethod {
    pub is_mut: bool,
    pub function: MetaFn,
}

pub struct MetaEnum {
    pub fields: HashMap<String, MetaEnumField>,
}

pub struct MetaEnumField {
    pub name: String,
    pub values: Option<MetaEnumFieldValues>,
}

pub enum MetaEnumFieldValues {
    Tuple(MetaEnumFieldTuple),
    Struct(MetaEnumFieldStruct),
}

pub struct MetaEnumFieldTuple {}

pub struct MetaEnumFieldStruct {}

