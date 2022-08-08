mod definitions;
mod import;
mod r#static;

pub use definitions::*;
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
    Instruction(Instruction),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Definition {
    Type(DefType),
    Function(DefFn),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DefType {
    Struct(DefStruct),
    Class(DefClass),
    Interface(DefInterface),
    Enum(DefEnum),
    Errors(DefErrors),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Generic {}

#[derive(Clone, Debug, PartialEq)]
pub struct Type {}

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Assignment(Assignment),
    Statement(Statement),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {}

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {}

#[derive(Clone, Debug, PartialEq)]
pub struct Statement {}


