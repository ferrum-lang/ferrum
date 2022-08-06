mod literal;
mod source_meta;

use anyhow::Result;

pub type Tokens = Vec<Token>;

pub enum Token {}

pub fn parse_tokens(text: String) -> Result<Tokens> { 
    let literals = literal::parse_literals(text);

    todo!("{literals:?}")
}

