use super::*;

#[derive(Clone, Debug)]
pub struct Macro {
    pub path: Path,
    pub delimiter: MacroDelimiter,
    pub tokens: proc_macro2::TokenStream,
}

impl PartialEq for Macro {
    fn eq(&self, other: &Self) -> bool {
        return self.path.eq(&other.path)
            && self.delimiter.eq(&other.delimiter)
            && self.tokens.to_string().eq(&other.tokens.to_string());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MacroDelimiter {
    Paren,
    Brace,
    Bracket,
}
