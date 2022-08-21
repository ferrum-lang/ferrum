use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct StaticConst {
    pub is_public: bool,
    pub name: String,
    pub typ: Box<Type>,
    pub expr: Box<Expr>,
}

