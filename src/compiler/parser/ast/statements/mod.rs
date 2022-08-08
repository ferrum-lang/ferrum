mod assignment;
mod expression;
mod r#loop;

pub use assignment::*;
pub use expression::*;
pub use r#loop::*;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Assignment(Assignment),
    Expression(Expression),
    Loop(Loop),
}

