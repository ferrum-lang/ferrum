mod built_in;
mod keyword;
mod literal;
mod token;

pub use built_in::BuiltInType;
pub use keyword::Keyword;
pub use literal::Literal;
pub use token::Token;

use crate::compiler::lexer::source_meta::SourceMeta;

#[derive(Clone, Debug, PartialEq)]
pub struct Tokens {
    pub value: Vec<TokenData>,
}

impl std::fmt::Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut line = 0;

        for token in self.value.iter() {
            let line_n = token.source_meta.lines.0;

            if line != line_n {
                line = line_n;

                let new_line = if line == 1 { "" } else { "\n\n" };

                if token.value == Token::NewLine {
                    if let Err(e) = write!(f, "{new_line}{line_n}. ") {
                        return Err(e);
                    }
                } else {
                    if let Err(e) = write!(f, "{new_line}{line_n}. {:?}", token.value) {
                        return Err(e);
                    }
                }
            } else if token.value != Token::NewLine {
                if let Err(e) = write!(f, " {:?}", token.value) {
                    return Err(e);
                }
            }
        }

        return Ok(());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenData {
    pub value: Token,
    pub source_meta: SourceMeta,
}


