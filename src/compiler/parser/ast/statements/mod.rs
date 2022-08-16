mod assignment;
mod binary_operation;
mod block_expr;
mod branch;
mod call;
mod closure;
mod construction;
mod list;
mod literal;
mod r#loop;
mod matches;
mod pattern;
mod range;
mod reference;
mod tuple;

pub use assignment::*;
pub use binary_operation::*;
pub use block_expr::*;
pub use branch::*;
pub use call::*;
pub use closure::*;
pub use construction::*;
pub use list::*;
pub use literal::*;
pub use matches::*;
pub use pattern::*;
pub use r#loop::*;
pub use range::*;
pub use reference::*;
pub use tuple::*;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Assignment(Assignment),
    Expression(Expression),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    FunctionCall(FunctionCall),
    MethodCall(MethodCall),
    Construction(Construction),       // Example { a, b }
    Reference(Reference),             // example, some.example, some::example
    Loop(Loop),                       // loop, loop-while, while, for
    Branch(Branch),                   // if/else, match, ternary
    BinaryOperation(BinaryOperation), // 1 + 2, 1 >= 2
    Matches(Matches),
    Closure(Closure), // () => {}
    Literal(Literal), // 1, "hello"
    Tuple(Tuple),     // (1, 2, 3)
    List(List),       // [1, 2, 3]
    ListIndexedItem(ListIndexedItem),
    Option(ExprOption),
    Result(ExprResult),
    Range(Range),     // 1..=10
    Block(BlockExpr),
    Mut(Box<Expression>),
    ListValueSpread(Box<Expression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprOption {
    Direct(ExprOptionDirect),
    Passed(ExprOptionPassed),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprOptionDirect {
    Some(Option<Box<Expression>>),
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprOptionPassed {
    pub reciever: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprResult {
    Direct(ExprResultDirect),
    Passed(ExprResultPassed),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprResultDirect {
    pub is_ok: bool,
    pub value: Option<Box<Expression>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprResultPassed {
    pub reciever: Option<Box<Expression>>,
}

