use super::{
  tokens::{Token, UnparsedToken},
  Error,
};

pub fn parse_tokens(unparsed_tokens: Vec<UnparsedToken>) -> Result<Vec<Token>, Error> {
  let mut tokens = vec![];
  let mut iter = unparsed_tokens.clone().into_iter().peekable();

  println!("{:?}", unparsed_tokens);

  // Iterate over unhandled unpased_tokens:
  while let Some(unparsed) = iter.next() {
    let literal = unparsed.get_literal().as_str();

    match literal {
      // Strings
      "\"" => {
        let mut buffer = String::new();

        let mut is_template_string = false;

        while let Some(peek) = iter.peek().cloned() {
          let peek = peek.get_literal().as_str();

          let mut break_loop = false;

          match peek {
            // End of string
            "\"" => {
              iter.next();

              let token = if is_template_string {
                Token::TemplateStringEnd(buffer.clone())
              } else {
                Token::PlainString(buffer.clone())
              };

              tokens.push(token);

              break_loop = true;
            }
            // Template string
            "{" => {
              let token = if is_template_string {
                Token::TemplateStringMiddle(buffer)
              } else {
                Token::TemplateStringStart(buffer)
              };

              tokens.push(token);

              is_template_string = true;

              tokens.push(Token::TemplateStringTemplateOpenBrace);
              iter.next();

              // Handle limited expressions within template string
              while let Some(unparsed) = iter.next() {
                let literal = unparsed.get_literal().as_str();

                match literal {
                  name if is_identifier_name(name) => {
                    let peek = iter.peek().expect("Incomplete template string!");
                    let peek = peek.get_literal().as_str();

                    match peek {
                      "::" => {
                        tokens.push(Token::TypeAccessName(String::from(name)));

                        tokens.push(Token::TypeAccessColons);
                        iter.next();
                      }
                      "." => {
                        tokens.push(Token::InstanceAccessName(String::from(name)));

                        tokens.push(Token::InstanceAccessPeriod);
                        iter.next();
                      }
                      "(" => {
                        tokens.push(Token::FunctionCallName(String::from(name)));

                        tokens.push(Token::FunctionCallOpenParenthesis);
                        iter.next();
                      }
                      _ => {
                        tokens.push(Token::InstanceReferenceName(String::from(name)));
                      }
                    }
                  }
                  number if is_numeric(number) => {
                    let peek = iter.peek().expect("Incomplete template string!");
                    let peek = peek.get_literal().as_str();

                    if peek == "." {
                      iter.next();

                      let peek = iter.peek().expect("Incomplete template string!");
                      let peek = peek.get_literal().as_str();

                      if !is_numeric(peek) {
                        todo!("Invalid float!");
                      }

                      tokens.push(Token::Float(format!("{}.{}", number, peek)));
                      iter.next();
                    } else {
                      tokens.push(Token::Int(String::from(number)));
                    }
                  }
                  "\"" => {
                    todo!("TODO: handle strings nested in a template string, or formally disallow.\n\nParsed Tokens: {:?}", tokens);
                  }
                  "'" => {
                    let mut buffer = String::new();

                    loop {
                      let peek = iter.next().expect(&*format!(
                        "Incomplete template string!\n\nParsed Tokens: {:?}",
                        tokens
                      ));
                      let peek = peek.get_literal().as_str();

                      match peek {
                        "'" => {
                          tokens.push(Token::Char(String::from(buffer)));
                          break;
                        }
                        _ => {
                          buffer.push_str(peek);
                        }
                      }
                    }
                  }
                  "," => {
                    tokens.push(Token::FunctionCallComma);
                  }
                  ")" => tokens.push(Token::FunctionCallCloseParenthesis),
                  // End of template string
                  "}" => {
                    tokens.push(Token::TemplateStringTemplateCloseBrace);
                    break;
                  }
                  _ if is_whitespace(literal) => {
                    tokens.push(Token::Whitespace);
                  }
                  _ => todo!(
                    "Unexpected token: {}\n\nParsed Tokens: {:?}",
                    literal,
                    tokens
                  ),
                }
              }

              buffer = String::new();
            }
            // Any non-special piece of the string
            _ => {
              buffer.push_str(peek);
              iter.next();
            }
          }

          if break_loop {
            break;
          }
        }
      }
      // Imports
      "import" => {
        tokens.push(Token::Import);

        let mut unparsed = iter.next().expect(&format!(
          "Unfinished import!\n\nParsed Tokens: {:?}",
          tokens
        ));
        let mut literal = unparsed.get_literal().as_str();

        while is_whitespace(literal) {
          tokens.push(Token::Whitespace);
          unparsed = iter.next().expect(&format!(
            "Unfinished import!\n\nParsed Tokens: {:?}",
            tokens
          ));
          literal = unparsed.get_literal().as_str();
        }

        match literal {
          "{" => {
            tokens.push(Token::DestructureOpenBrace);

            todo!("Handle destructured import\n\nParsed Tokens: {:?}", tokens);
          }
          _ if is_whitespace(literal) => {
            tokens.push(Token::Whitespace);
          }
          _ => todo!(
            "Unexpected token: {}\n\nParsed Tokens: {:?}",
            literal,
            tokens
          ),
        }
      }
      _ if is_whitespace(literal) => {
        tokens.push(Token::Whitespace);
      }
      _ => todo!(
        "Unexpected token: {}\n\nParsed Tokens: {:?}",
        literal,
        tokens
      ),
    }
  }

  todo!("parse_tokens\n\nParsed Tokens: {:?}", tokens);
}

fn is_whitespace(string: &str) -> bool {
  string.trim().is_empty()
}

fn is_identifier_name(string: &str) -> bool {
  let mut is_first = true;

  for c in string.chars() {
    if is_first && !c.is_alphabetic() && c != '_' {
      return false;
    }

    if !is_first && !c.is_alphanumeric() && c != '_' {
      return false;
    }

    is_first = false;
  }

  return true;
}

fn is_numeric(string: &str) -> bool {
  string.chars().all(char::is_numeric)
}
