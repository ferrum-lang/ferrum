use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ItemEnum {
    pub is_public: bool,
    pub name: String,
    pub generics: Generics,
    pub variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub fields: EnumVariantFields,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnumVariantFields {
    Named(EnumVariantNamedFields),
    Unnamed(EnumVariantUnnamedFields),
    Unit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariantNamedFields {
    pub fields: Vec<EnumVariantNamedField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariantNamedField {
    pub name: String,
    pub typ: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariantUnnamedFields {
    pub fields: Vec<EnumVariantUnnamedField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariantUnnamedField {
    pub typ: Type,
}

