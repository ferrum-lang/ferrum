use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexicalError {
    #[error("Unexpected character: '{c}' on line {line}")]
    UnexpectedCharacter { c: char, line: usize },
}

