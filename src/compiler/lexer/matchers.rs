pub enum TokenMatcher {
  SingleChar(char),
  SinglePredicate(Box<dyn Fn(char) -> bool>),
  BufferedChar(char),
  BufferedPredicate(Box<dyn Fn(&String, char) -> bool>),
}

impl std::fmt::Debug for TokenMatcher {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Self::SingleChar(c) => write!(fmt, "SingleChar(\"{}\")", c),
      Self::SinglePredicate(_) => write!(fmt, "SinglePredicate(<predicate>)"),
      Self::BufferedChar(c) => write!(fmt, "BufferedChar(\"{}\")", c),
      Self::BufferedPredicate(_) => write!(fmt, "BufferedPredicate(<predicate>)"),
    }
  }
}

pub fn get_token_matchers() -> Vec<TokenMatcher> {
  vec![
    TokenMatcher::BufferedPredicate(Box::new(|_, c| c.is_whitespace())),
    TokenMatcher::BufferedPredicate(Box::new(|_, c| c.is_alphabetic())),
    TokenMatcher::BufferedPredicate(Box::new(|_, c| c.is_numeric())),
    TokenMatcher::BufferedChar('_'),
    TokenMatcher::SingleChar('{'),
    TokenMatcher::SingleChar('}'),
    TokenMatcher::SingleChar(';'),
    TokenMatcher::SingleChar('"'),
    TokenMatcher::SingleChar('\''),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '\\', // matches "\"
      "\\" => true,    // matches "\*" where * is anything
      _ => false,
    })),
    TokenMatcher::SingleChar('('),
    TokenMatcher::SingleChar(')'),
    TokenMatcher::SingleChar('['),
    TokenMatcher::SingleChar(','),
    TokenMatcher::SingleChar(']'),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == ':',  // matches ":"
      ":" => c == ':', // matches "::"
      _ => false,
    })),
    TokenMatcher::SingleChar('.'),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '=',              // matches "="
      "=" => c == '=' || c == '>', // matches "==" or "=>"
      _ => false,
    })),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '!',  // matches "!"
      "!" => c == '=', // matches "!="
      _ => false,
    })),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '+',  // matches "+"
      "+" => c == '=', // matches "+="
      _ => false,
    })),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '-',  // matches "-"
      "-" => c == '=', // matches "-="
      _ => false,
    })),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '*',  // matches "*"
      "*" => c == '=', // matches "*="
      _ => false,
    })),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '/',  // matches "/"
      "/" => c == '=', // matches "/="
      _ => false,
    })),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '%',  // matches "%"
      "%" => c == '=', // matches "%="
      _ => false,
    })),
  ]
}
