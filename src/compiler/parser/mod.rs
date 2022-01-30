use super::{
  tokens::{Token, UnparsedToken},
  Error,
};

pub fn parse_tokens(mut unparsed_tokens: Vec<UnparsedToken>) -> Result<Vec<Token>, Error> {
  let mut tokens = vec![];

  unparsed_tokens.reverse();

  while let Some(unparsed) = unparsed_tokens.pop() {
    let literal = unparsed.get_literal().as_str();

    match literal {
      ";" => tokens.push(Token::Semicolon),
      "public" => tokens.push(Token::Public),
      "\"" => parse_string(&mut unparsed_tokens, &mut tokens, literal)?,
      "import" => parse_import(&mut unparsed_tokens, &mut tokens, literal)?,
      "function" => parse_function(&mut unparsed_tokens, &mut tokens, literal)?,
      _ if is_whitespace(literal) => parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?,
      _ => todo!(
        "Unexpected token: {}\n\nParsed Tokens: {:?}",
        literal,
        tokens
      ),
    }
  }

  return Ok(tokens);
}

fn parse_string(
  mut unparsed_tokens: &mut Vec<UnparsedToken>,
  mut tokens: &mut Vec<Token>,
  _literal: &str,
) -> Result<(), Error> {
  let mut buffer = String::new();

  let mut is_template_string = false;

  while let Some(peek) = unparsed_tokens.last() {
    let peek = peek.get_literal().as_str();

    let mut break_loop = false;

    match peek {
      // End of string
      "\"" => {
        unparsed_tokens.pop();

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
        unparsed_tokens.pop();

        // Handle limited expressions within template string
        while let Some(unparsed) = unparsed_tokens.pop() {
          let literal = unparsed.get_literal().as_str();

          match literal {
            name if is_identifier_name(name) => {
              let peek = unparsed_tokens.last().expect("Incomplete template string!");
              let peek = peek.get_literal().as_str();

              match peek {
                "::" => {
                  tokens.push(Token::TypeAccessName(String::from(name)));

                  tokens.push(Token::TypeAccessColons);
                  unparsed_tokens.pop();
                }
                "." => {
                  tokens.push(Token::InstanceAccessName(String::from(name)));

                  tokens.push(Token::InstanceAccessPeriod);
                  unparsed_tokens.pop();
                }
                "(" => {
                  tokens.push(Token::FunctionCallName(String::from(name)));

                  tokens.push(Token::FunctionCallOpenParenthesis);
                  unparsed_tokens.pop();
                }
                _ => {
                  tokens.push(Token::InstanceReferenceName(String::from(name)));
                }
              }
            }
            number if is_numeric(number) => {
              let peek = unparsed_tokens.last().expect("Incomplete template string!");
              let peek = peek.get_literal().as_str();

              if peek == "." {
                unparsed_tokens.pop();

                let peek = unparsed_tokens.last().expect("Incomplete template string!");
                let peek = peek.get_literal().as_str();

                if !is_numeric(peek) {
                  todo!("Invalid float!");
                }

                tokens.push(Token::Float(format!("{}.{}", number, peek)));
                unparsed_tokens.pop();
              } else {
                tokens.push(Token::Int(String::from(number)));
              }
            }
            // Another string nested within current template string
            "\"" => parse_string(&mut unparsed_tokens, &mut tokens, "\"")?,
            "'" => {
              let mut buffer = String::new();

              loop {
                let peek = unparsed_tokens.pop().expect(&*format!(
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
              parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?
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
        unparsed_tokens.pop();
      }
    }

    if break_loop {
      break;
    }
  }

  return Ok(());
}

fn parse_import(
  mut unparsed_tokens: &mut Vec<UnparsedToken>,
  mut tokens: &mut Vec<Token>,
  _literal: &str,
) -> Result<(), Error> {
  tokens.push(Token::Import);

  loop {
    let unparsed = unparsed_tokens.pop().expect(&format!(
      "Unfinished import!\n\nParsed Tokens: {:?}",
      tokens
    ));
    let literal = unparsed.get_literal().as_str();

    match literal {
      // Destructured
      "{" => {
        tokens.push(Token::DestructureOpenBrace);

        while let Some(unparsed) = unparsed_tokens.pop() {
          let literal = unparsed.get_literal().as_str();

          match literal {
            name if is_identifier_name(name) => {
              tokens.push(Token::DestructureField(name.to_string()))
            }
            ":" => {
              tokens.push(Token::DestructureAliasColon);

              let mut unparsed = unparsed_tokens.pop().expect(&format!(
                "Unfinished import!\n\nParsed Tokens: {:?}",
                tokens
              ));
              let mut literal = unparsed.get_literal().as_str();

              if is_whitespace(literal) {
                parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?;

                unparsed = unparsed_tokens.pop().expect(&format!(
                  "Unfinished import!\n\nParsed Tokens: {:?}",
                  tokens
                ));
                literal = unparsed.get_literal().as_str();
              }

              if !is_identifier_name(literal) {
                todo!("Unfinished import!\n\nParsed Tokens: {:?}", tokens);
              }

              tokens.push(Token::DestructureAliasName(literal.to_string()));
            }
            "," => tokens.push(Token::DestructureComma),
            "}" => {
              tokens.push(Token::DestructureCloseBrace);
              break;
            }
            _ if is_whitespace(literal) => {
              parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?
            }
            _ => todo!(
              "Unexpected token: {}\n\nParsed Tokens: {:?}",
              literal,
              tokens
            ),
          }
        }

        break;
      }
      _ if is_whitespace(literal) => parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?,
      _ => todo!(
        "Unexpected token: {}\n\nParsed Tokens: {:?}",
        literal,
        tokens
      ),
    }
  }

  loop {
    let unparsed = unparsed_tokens.pop().expect(&format!(
      "Unfinished import!\n\nParsed Tokens: {:?}",
      tokens
    ));
    let literal = unparsed.get_literal().as_str();

    match literal {
      "from" => tokens.push(Token::ImportFrom),
      "\"" => {
        loop {
          let unparsed = unparsed_tokens.pop().expect(&format!(
            "Unfinished import!\n\nParsed Tokens: {:?}",
            tokens
          ));
          let literal = unparsed.get_literal().as_str();
          match literal {
            _ if is_import_source(literal) => {
              tokens.push(Token::ImportSource(literal.to_string()));

              let unparsed = unparsed_tokens.pop().expect(&format!(
                "Unfinished import!\n\nParsed Tokens: {:?}",
                tokens
              ));
              let literal = unparsed.get_literal().as_str();

              if literal != "\"" {
                todo!("Unfinished import!\n\nParsed Tokens: {:?}", tokens);
              }

              break;
            }
            _ if is_whitespace(literal) => {
              parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?
            }
            _ => todo!(
              "Unexpected token: {}\n\nParsed Tokens: {:?}",
              literal,
              tokens
            ),
          }
        }

        break;
      }
      _ if is_whitespace(literal) => parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?,
      _ => todo!(
        "Unexpected token: {}\n\nParsed Tokens: {:?}",
        literal,
        tokens
      ),
    }
  }

  return Ok(());
}

fn parse_function(
  mut unparsed_tokens: &mut Vec<UnparsedToken>,
  mut tokens: &mut Vec<Token>,
  _literal: &str,
) -> Result<(), Error> {
  tokens.push(Token::Function);

  loop {
    let unparsed = unparsed_tokens.pop().expect(&format!(
      "Unfinished function!\n\nParsed Tokens: {:?}",
      tokens
    ));
    let literal = unparsed.get_literal().as_str();

    match literal {
      name if is_identifier_name(name) => {
        tokens.push(Token::FunctionName(name.to_string()));

        expect_next(&mut unparsed_tokens, &mut tokens, &|l| l == "(", true)?;
        parse_function_params(&mut unparsed_tokens, &mut tokens, "(")?;

        expect_next(&mut unparsed_tokens, &mut tokens, &|l| l == "{", true)?;
        parse_function_body(&mut unparsed_tokens, &mut tokens, "{")?;

        break;
      }
      _ if is_whitespace(literal) => parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?,
      _ => todo!(
        "Unexpected token: {}\n\nParsed Tokens: {:?}",
        literal,
        tokens
      ),
    }
  }

  return Ok(());
}

fn parse_function_params(
  mut unparsed_tokens: &mut Vec<UnparsedToken>,
  mut tokens: &mut Vec<Token>,
  _literal: &str,
) -> Result<(), Error> {
  tokens.push(Token::FunctionParamsOpenParenthesis);

  loop {
    let unparsed = unparsed_tokens.pop().expect(&format!(
      "Unfinished function!\n\nParsed Tokens: {:?}",
      tokens
    ));
    let literal = unparsed.get_literal().as_str();

    match literal {
      name if is_identifier_name(name) => {
        tokens.push(Token::FunctionParamsParamName(name.to_string()));

        expect_next(&mut unparsed_tokens, &mut tokens, &|l| l == ":", true)?;
        tokens.push(Token::FunctionParamsParamTypeColon);

        loop {
          let unparsed = unparsed_tokens.pop().expect(&format!(
            "Unfinished function!\n\nParsed Tokens: {:?}",
            tokens
          ));
          let literal = unparsed.get_literal().as_str();

          match literal {
            name if is_identifier_name(name) => {
              tokens.push(Token::FunctionParamsParamTypeName(name.to_string()))
            }
            "mutable" => tokens.push(Token::FunctionParamsParamTypeMutable),
            "shared" => tokens.push(Token::FunctionParamsParamTypeShared),
            "&" => tokens.push(Token::FunctionParamsParamTypeBorrowed),
            "," => {
              tokens.push(Token::FunctionParamsComma);
              break;
            }
            _ if is_whitespace(literal) => {
              parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?
            }
            _ => todo!(
              "Unexpected token: {}\n\nParsed Tokens: {:?}",
              literal,
              tokens
            ),
          }
        }
      }
      ")" => {
        tokens.push(Token::FunctionParamsCloseParenthesis);

        loop {
          let unparsed = unparsed_tokens.pop().expect(&format!(
            "Unfinished function!\n\nParsed Tokens: {:?}",
            tokens
          ));
          let literal = unparsed.get_literal().as_str();

          match literal {
            ":" => {
              tokens.push(Token::FunctionReturnTypeColon);

              loop {
                let unparsed = unparsed_tokens.pop().expect(&format!(
                  "Unfinished function!\n\nParsed Tokens: {:?}",
                  tokens
                ));
                let literal = unparsed.get_literal().as_str();

                match literal {
                  name if is_identifier_name(name) => {
                    tokens.push(Token::FunctionParamsParamTypeName(name.to_string()))
                  }
                  "mutable" => tokens.push(Token::FunctionParamsParamTypeMutable),
                  "shared" => tokens.push(Token::FunctionParamsParamTypeShared),
                  "&" => tokens.push(Token::FunctionParamsParamTypeBorrowed),
                  "{" => {
                    unparsed_tokens.push(unparsed);
                    break;
                  }
                  _ if is_whitespace(literal) => {
                    parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?
                  }
                  _ => todo!(
                    "Unexpected token: {}\n\nParsed Tokens: {:?}",
                    literal,
                    tokens
                  ),
                }
              }
            }
            "{" => {
              unparsed_tokens.push(unparsed);
              break;
            }
            _ if is_whitespace(literal) => {
              parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?
            }
            _ => todo!(
              "Unexpected token: {}\n\nParsed Tokens: {:?}",
              literal,
              tokens
            ),
          }
        }

        break;
      }
      _ if is_whitespace(literal) => parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?,
      _ => todo!(
        "Unexpected token: {}\n\nParsed Tokens: {:?}",
        literal,
        tokens
      ),
    }
  }

  return Ok(());
}

fn parse_function_body(
  mut unparsed_tokens: &mut Vec<UnparsedToken>,
  mut tokens: &mut Vec<Token>,
  _literal: &str,
) -> Result<(), Error> {
  tokens.push(Token::FunctionExpressionsOpenBrace);

  loop {
    let unparsed = unparsed_tokens.pop().expect(&format!(
      "Unfinished function!\n\nParsed Tokens: {:?}",
      tokens
    ));
    let literal = unparsed.get_literal().as_str();

    match literal {
      "\"" => parse_string(&mut unparsed_tokens, &mut tokens, literal)?,
      name if is_identifier_name(name) => {
        let mut unparsed_peek = unparsed_tokens.last().expect(&format!(
          "Unfinished function!\n\nParsed Tokens: {:?}",
          tokens
        ));
        let mut peek = unparsed_peek.get_literal().as_str();

        while is_whitespace(peek) {
          unparsed_tokens.pop();

          unparsed_peek = unparsed_tokens.last().expect(&format!(
            "Unfinished function!\n\nParsed Tokens: {:?}",
            tokens
          ));
          peek = unparsed_peek.get_literal().as_str();
        }

        match peek {
          "::" => {
            tokens.push(Token::TypeAccessName(name.to_string()));

            tokens.push(Token::TypeAccessDoubleSemicolon);
            unparsed_tokens.pop();
          }
          "." => {
            tokens.push(Token::InstanceAccessName(name.to_string()));

            tokens.push(Token::InstanceAccessPeriod);
            unparsed_tokens.pop();
          }
          "(" => {
            tokens.push(Token::FunctionCallName(name.to_string()));

            tokens.push(Token::FunctionCallOpenParenthesis);
            unparsed_tokens.pop();
          }
          _ if is_whitespace(literal) => {
            parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?
          }
          _ => tokens.push(Token::InstanceReferenceName(String::from(name))),
        }
      }
      "let" => {
        tokens.push(Token::Let);
        todo!();
      }
      "const" => {
        tokens.push(Token::Const);
        todo!();
      }
      ")" => tokens.push(Token::FunctionCallCloseParenthesis),
      ";" => tokens.push(Token::Semicolon),
      "}" => {
        tokens.push(Token::FunctionExpressionsCloseBrace);
        break;
      }
      _ if is_whitespace(literal) => parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?,
      _ => todo!(
        "Unexpected token: {}\n\nParsed Tokens: {:?}",
        literal,
        tokens
      ),
    }
  }

  return Ok(());
}

fn parse_whitespace(
  unparsed_tokens: &mut Vec<UnparsedToken>,
  _tokens: &mut Vec<Token>,
  _literal: &str,
) -> Result<(), Error> {
  // tokens.push(Token::Whitespace);

  while let Some(unparsed) = unparsed_tokens.last() {
    let literal = unparsed.get_literal().as_str();

    if !is_whitespace(literal) {
      break;
    }

    unparsed_tokens.pop();
  }

  return Ok(());
}

fn expect_next(
  mut unparsed_tokens: &mut Vec<UnparsedToken>,
  mut tokens: &mut Vec<Token>,
  expected: &dyn Fn(&str) -> bool,
  allow_whitespace: bool,
) -> Result<(), Error> {
  let mut unparsed = unparsed_tokens.pop().expect(&format!(
    "Unfinished function!\n\nParsed Tokens: {:?}",
    tokens
  ));
  let mut literal = unparsed.get_literal().as_str();

  if allow_whitespace && is_whitespace(literal) {
    parse_whitespace(&mut unparsed_tokens, &mut tokens, literal)?;

    unparsed = unparsed_tokens.pop().expect(&format!(
      "Unfinished function!\n\nParsed Tokens: {:?}",
      tokens
    ));
    literal = unparsed.get_literal().as_str();
  }

  if !expected(literal) {
    todo!(
      "Unexpected token: {}\n\nParsed Tokens: {:?}",
      literal,
      tokens
    );
  }

  return Ok(());
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

fn is_import_source(string: &str) -> bool {
  let mut is_first = true;

  for c in string.chars() {
    if is_first && !c.is_alphabetic() && c != '_' && c != '@' {
      return false;
    }

    if !is_first && !c.is_alphanumeric() && c != '_' && c != '@' && c != '/' {
      return false;
    }

    is_first = false;
  }

  return true;
}

fn is_numeric(string: &str) -> bool {
  string.chars().all(char::is_numeric)
}
