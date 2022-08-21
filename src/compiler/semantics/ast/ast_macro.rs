use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Macro {
    pub ident: String,
    pub path: Path,
    pub delimiter: MacroDelimiter,
    pub values: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MacroDelimiter {
    Paren,
    Brace,
    Bracket,
}
