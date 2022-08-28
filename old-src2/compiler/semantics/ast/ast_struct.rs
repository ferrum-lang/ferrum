use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ItemStruct {
    pub is_public: bool,
    pub name: String,
    pub generics: Generics,
    pub fields: Fields,
}

