#[derive(Clone)]
pub struct Token {
    literal: String,
}

impl Token {
    pub fn new<S: Into<String>>(literal: S) -> Self {
        Self {
            literal: literal.into(),
        }
    }

    pub fn get_literal(&self) -> &String {
        return &self.literal;
    }

    pub fn take_literal(self) -> String {
        return self.literal;
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "Token(\"{}\")", self.literal.replace("\n", "\\n"))
    }
}
