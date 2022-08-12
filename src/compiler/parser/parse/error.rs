use super::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(TokenData),

    #[error("Missing expected token: {0:?}")]
    MissingExpectedToken(Option<Token>),
}

