mod definitions;
mod statements;
mod import;
mod r#static;

pub use definitions::*;
pub use statements::*;
pub use import::*;
pub use r#static::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AST {
    pub imports: Vec<Import>,
    pub statics: Vec<Static>,
    pub nodes: Vec<RootNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RootNode {
    Definition(Definition),
    Statement(Statement),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Type {}

#[derive(Clone, Debug, PartialEq)]
pub struct Condition {}

