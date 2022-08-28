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
    pub fields: Fields,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Fields {
    Named(NamedFields),
    Unnamed(UnnamedFields),
    Unit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamedFields {
    pub fields: Vec<NamedField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamedField {
    pub name: String,
    pub typ: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnnamedFields {
    pub fields: Vec<UnnamedField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnnamedField {
    pub typ: Type,
}

