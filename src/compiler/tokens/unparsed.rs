pub struct UnparsedToken {
  literal: String,
}

impl UnparsedToken {
  pub fn new<S: Into<String>>(literal: S) -> Self {
    Self {
      literal: literal.into(),
    }
  }

  pub fn get_literal(&self) -> &String {
    return &self.literal;
  }
}

impl std::fmt::Debug for UnparsedToken {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    write!(fmt, "Unparsed(\"{}\")", self.literal)
  }
}
