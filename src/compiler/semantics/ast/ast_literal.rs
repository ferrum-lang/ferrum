#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Char(char),
    Int(String),
    Float(String),
    Bool(bool),
}

