use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ItemConst {
    pub is_public: bool,
    pub name: String,
    pub typ: Box<Type>,
    pub expr: Box<Expr>,
}

