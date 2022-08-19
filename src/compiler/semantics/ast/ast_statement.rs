use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Local(Local),
    Item(Item),
    Expr(Expr),
    Semi(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Local {
    pub pattern: Pattern,
    pub init: Option<Box<Expr>>,
}

