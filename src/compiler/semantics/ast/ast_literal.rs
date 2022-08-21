#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Char(String),
    Int(String),
    Float(String),
    Bool(bool),
}

