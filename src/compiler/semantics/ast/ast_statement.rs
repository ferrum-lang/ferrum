use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Local(Local),
    Const(Const),
    Item(Item),
    Expr(Expr),
    Semi(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Local {
    pub pattern: Pattern,
    pub init: Option<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Const {
    pub name: String,
    pub typ: Box<Type>,
    pub expr: Box<Expr>,
}

