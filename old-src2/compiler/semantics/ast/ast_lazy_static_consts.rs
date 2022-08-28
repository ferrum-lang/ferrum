use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LazyStaticConst {
    pub name: String,
    pub typ: Box<Type>,
    pub value: Box<Expr>,
}

