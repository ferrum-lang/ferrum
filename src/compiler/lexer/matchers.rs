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
      Self::SinglePredicate(_) => write!(fmt, "SinglePredicate(< Fn(char) -> bool >)"),
      Self::BufferedChar(c) => write!(fmt, "BufferedChar(\"{}\")", c),
      Self::BufferedPredicate(_) => write!(fmt, "BufferedPredicate(< Fn(&String, char) -> bool >)"),
    }
  }
}

pub fn get_token_matchers() -> Vec<TokenMatcher> {
  vec![
    TokenMatcher::BufferedPredicate(Box::new(|_, c| c.is_whitespace() && c != '\n')),
    TokenMatcher::SingleChar('\n'),
    TokenMatcher::BufferedPredicate(Box::new(|_, c| c.is_numeric())),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c.is_alphabetic() || c == '_', // tokens and identifiers must start with a letter or underscore
      _ => c.is_alphanumeric() || c == '_', // then can contain letters, numbers, and underscores
    })),
    TokenMatcher::SingleChar('{'),
    TokenMatcher::SingleChar('}'),
    TokenMatcher::SingleChar(';'),
    TokenMatcher::SingleChar('"'),
    TokenMatcher::SingleChar('?'),
    TokenMatcher::SingleChar('~'),
    TokenMatcher::SingleChar('\''),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '\\', // matches "\"
      "\\" => true,    // matches "\*" where * is any char
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
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '.',  // matches "."
      "." => c == '.', // matches ".."
      _ => false,
    })),
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
      "" => c == '&',  // matches "&"
      "&" => c == '&', // matches "&&"
      _ => false,
    })),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '|',  // matches "|"
      "|" => c == '|', // matches "||"
      _ => false,
    })),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '<',  // matches "<"
      "<" => c == '=', // matches "<="
      _ => false,
    })),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '>',  // matches ">"
      ">" => c == '=', // matches ">="
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
      "" => c == '/',                                                 // matches "/"
      "/" => c == '=' || c == '/' || c == '*', // matches "/=" or "//" or "/*"
      b if b.starts_with("//") => c != '\n',   // matches comments starting with "//"
      b if b.starts_with("/*") => b.len() <= 3 || !b.ends_with("*/"), // matches comments starting with "/*"
      _ => false,
    })),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '%',  // matches "%"
      "%" => c == '=', // matches "%="
      _ => false,
    })),
    TokenMatcher::BufferedPredicate(Box::new(|buffer, c| match buffer.as_str() {
      "" => c == '^',  // matches "^"
      "^" => c == '=', // matches "^="
      _ => false,
    })),
  ]
}
