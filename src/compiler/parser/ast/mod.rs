mod definitions;
mod statements;
mod import;
mod r#static;
mod r#type;

pub use definitions::*;
pub use statements::*;
pub use import::*;
pub use r#static::*;
pub use r#type::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AST {
    pub imports: Vec<Import>,
    pub static_consts: Vec<StaticConst>,
    pub nodes: Vec<RootNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RootNode {
    Definition(Definition),
    Statement(Statement),
}

