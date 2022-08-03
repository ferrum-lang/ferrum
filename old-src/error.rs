use std::fmt;

#[derive(Clone, Default)]
pub struct Error {
    message: Option<String>,
}

impl Error {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: Some(message.into()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self.message {
            Some(message) => write!(fmt, "{}", message),
            _ => write!(fmt, "Error!"),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self)
    }
}

impl std::error::Error for Error {}
