mod class;
mod r#enum;
mod errors;
mod function;
mod generics;
mod interface;
mod r#struct;

pub use class::*;
pub use r#enum::*;
pub use errors::*;
pub use function::*;
pub use generics::*;
pub use interface::*;
pub use r#struct::*;

use super::*;

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

