use super::{tokens::Token, Error, Symbol};

pub fn symolize_tokens(mut tokens: Vec<Token>) -> Result<Vec<Symbol>, Error> {
  let mut symbols = vec![];

  tokens.reverse();

  while let Some(token) = tokens.pop() {
    let literal = token.get_literal().as_str();

    match literal {
      ";" => symbols.push(Symbol::Semicolon),
      "public" => symbols.push(Symbol::Public),
      "\"" => symbolize_string(&mut tokens, &mut symbols, literal)?,
      "import" => symbolize_import(&mut tokens, &mut symbols, literal)?,
      "function" => symbolize_function(&mut tokens, &mut symbols, literal)?,
      _ if is_whitespace(literal) => symbolize_whitespace(&mut tokens, &mut symbols, literal)?,
      _ => todo!("Unexpected token: {}\n\nSymbols : {:?}", literal, symbols),
    }
  }

  return Ok(symbols);
}

fn symbolize_string(
  mut tokens: &mut Vec<Token>,
  mut symbols: &mut Vec<Symbol>,
  _literal: &str,
) -> Result<(), Error> {
  let mut buffer = String::new();

  let mut is_template_string = false;

  while let Some(peek) = tokens.last() {
    let peek = peek.get_literal().as_str();

    let mut break_loop = false;

    match peek {
      // End of string
      "\"" => {
        tokens.pop();

        let symbol = if is_template_string {
          Symbol::TemplateStringEnd(buffer.clone())
        } else {
          Symbol::PlainString(buffer.clone())
        };

        symbols.push(symbol);

        break_loop = true;
      }
      // Template string
      "{" => {
        let symbol = if is_template_string {
          Symbol::TemplateStringMiddle(buffer)
        } else {
          Symbol::TemplateStringStart(buffer)
        };

        symbols.push(symbol);

        is_template_string = true;

        symbols.push(Symbol::TemplateStringTemplateOpenBrace);
        tokens.pop();

        // Handle limited expressions within template string
        while let Some(token) = tokens.pop() {
          let literal = token.get_literal().as_str();

          match literal {
            name if is_identifier_name(name) => {
              let peek = tokens.last().expect("Incomplete template string!");
              let peek = peek.get_literal().as_str();

              match peek {
                "::" => {
                  symbols.push(Symbol::TypeAccessName(String::from(name)));

                  symbols.push(Symbol::TypeAccessColons);
                  tokens.pop();
                }
                "." => {
                  symbols.push(Symbol::InstanceAccessName(String::from(name)));

                  symbols.push(Symbol::InstanceAccessPeriod);
                  tokens.pop();
                }
                "(" => {
                  symbols.push(Symbol::FunctionCallName(String::from(name)));

                  symbols.push(Symbol::FunctionCallOpenParenthesis);
                  tokens.pop();
                }
                _ => {
                  symbols.push(Symbol::InstanceReferenceName(String::from(name)));
                }
              }
            }
            number if is_numeric(number) => {
              let peek = tokens.last().expect("Incomplete template string!");
              let peek = peek.get_literal().as_str();

              if peek == "." {
                tokens.pop();

                let peek = tokens.last().expect("Incomplete template string!");
                let peek = peek.get_literal().as_str();

                if !is_numeric(peek) {
                  todo!("Invalid float!");
                }

                symbols.push(Symbol::Float(format!("{}.{}", number, peek)));
                tokens.pop();
              } else {
                symbols.push(Symbol::Int(String::from(number)));
              }
            }
            // Another string nested within current template string
            "\"" => symbolize_string(&mut tokens, &mut symbols, "\"")?,
            "'" => {
              let mut buffer = String::new();

              loop {
                let peek = tokens.pop().expect(&*format!(
                  "Incomplete template string!\n\nSymbols: {:?}",
                  symbols
                ));
                let peek = peek.get_literal().as_str();

                match peek {
                  "'" => {
                    symbols.push(Symbol::Char(String::from(buffer)));
                    break;
                  }
                  _ => {
                    buffer.push_str(peek);
                  }
                }
              }
            }
            "," => {
              symbols.push(Symbol::FunctionCallComma);
            }
            ")" => symbols.push(Symbol::FunctionCallCloseParenthesis),
            // End of template string
            "}" => {
              symbols.push(Symbol::TemplateStringTemplateCloseBrace);
              break;
            }
            _ if is_whitespace(literal) => {
              symbolize_whitespace(&mut tokens, &mut symbols, literal)?
            }
            _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
          }
        }

        buffer = String::new();
      }
      // Any non-special piece of the string
      _ => {
        buffer.push_str(peek);
        tokens.pop();
      }
    }

    if break_loop {
      break;
    }
  }

  return Ok(());
}

fn symbolize_import(
  mut tokens: &mut Vec<Token>,
  mut symbols: &mut Vec<Symbol>,
  _literal: &str,
) -> Result<(), Error> {
  symbols.push(Symbol::Import);

  loop {
    let token = tokens
      .pop()
      .expect(&format!("Unfinished import!\n\nSymbols: {:?}", symbols));
    let literal = token.get_literal().as_str();

    match literal {
      // Destructured
      "{" => {
        symbols.push(Symbol::DestructureOpenBrace);

        while let Some(token) = tokens.pop() {
          let literal = token.get_literal().as_str();

          match literal {
            name if is_identifier_name(name) => {
              symbols.push(Symbol::DestructureField(name.to_string()))
            }
            ":" => {
              symbols.push(Symbol::DestructureAliasColon);

              let mut token = tokens
                .pop()
                .expect(&format!("Unfinished import!\n\nSymbols: {:?}", symbols));
              let mut literal = token.get_literal().as_str();

              if is_whitespace(literal) {
                symbolize_whitespace(&mut tokens, &mut symbols, literal)?;

                token = tokens.pop().expect(&format!(
                  "Unfinished import!\n\nParsed Tokens: {:?}",
                  symbols
                ));
                literal = token.get_literal().as_str();
              }

              if !is_identifier_name(literal) {
                todo!("Unfinished import!\n\nSymbols: {:?}", symbols);
              }

              symbols.push(Symbol::DestructureAliasName(literal.to_string()));
            }
            "," => symbols.push(Symbol::DestructureComma),
            "}" => {
              symbols.push(Symbol::DestructureCloseBrace);
              break;
            }
            _ if is_whitespace(literal) => {
              symbolize_whitespace(&mut tokens, &mut symbols, literal)?
            }
            _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
          }
        }

        break;
      }
      _ if is_whitespace(literal) => symbolize_whitespace(&mut tokens, &mut symbols, literal)?,
      _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
    }
  }

  loop {
    let token = tokens
      .pop()
      .expect(&format!("Unfinished import!\n\nSymbols: {:?}", symbols));
    let literal = token.get_literal().as_str();

    match literal {
      "from" => symbols.push(Symbol::ImportFrom),
      "\"" => {
        loop {
          let token = tokens
            .pop()
            .expect(&format!("Unfinished import!\n\nSymbols: {:?}", symbols));
          let literal = token.get_literal().as_str();
          match literal {
            _ if is_import_source(literal) => {
              symbols.push(Symbol::ImportSource(literal.to_string()));

              let token = tokens
                .pop()
                .expect(&format!("Unfinished import!\n\nSymbols: {:?}", symbols));
              let literal = token.get_literal().as_str();

              if literal != "\"" {
                todo!("Unfinished import!\n\nSymbols: {:?}", symbols);
              }

              break;
            }
            _ if is_whitespace(literal) => {
              symbolize_whitespace(&mut tokens, &mut symbols, literal)?
            }
            _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
          }
        }

        break;
      }
      _ if is_whitespace(literal) => symbolize_whitespace(&mut tokens, &mut symbols, literal)?,
      _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
    }
  }

  return Ok(());
}

fn symbolize_function(
  mut tokens: &mut Vec<Token>,
  mut symbols: &mut Vec<Symbol>,
  _literal: &str,
) -> Result<(), Error> {
  symbols.push(Symbol::Function);

  loop {
    let token = tokens
      .pop()
      .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
    let literal = token.get_literal().as_str();

    match literal {
      name if is_identifier_name(name) => {
        symbols.push(Symbol::FunctionName(name.to_string()));

        expect_next(&mut tokens, &mut symbols, &|l| l == "(", true)?;
        symbolize_function_params(&mut tokens, &mut symbols, "(")?;

        expect_next(&mut tokens, &mut symbols, &|l| l == "{", true)?;
        symbolize_function_body(&mut tokens, &mut symbols, "{")?;

        break;
      }
      _ if is_whitespace(literal) => symbolize_whitespace(&mut tokens, &mut symbols, literal)?,
      _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
    }
  }

  return Ok(());
}

fn symbolize_function_params(
  mut tokens: &mut Vec<Token>,
  mut symbols: &mut Vec<Symbol>,
  _literal: &str,
) -> Result<(), Error> {
  symbols.push(Symbol::FunctionParamsOpenParenthesis);

  loop {
    let token = tokens
      .pop()
      .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
    let literal = token.get_literal().as_str();

    match literal {
      name if is_identifier_name(name) => {
        symbols.push(Symbol::FunctionParamsParamName(name.to_string()));

        expect_next(&mut tokens, &mut symbols, &|l| l == ":", true)?;
        symbols.push(Symbol::FunctionParamsParamTypeColon);

        loop {
          let token = tokens
            .pop()
            .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
          let literal = token.get_literal().as_str();

          match literal {
            name if is_identifier_name(name) => {
              symbols.push(Symbol::FunctionParamsParamTypeName(name.to_string()));
              break;
            }
            "mutable" => symbols.push(Symbol::FunctionParamsParamTypeMutable),
            "shared" => symbols.push(Symbol::FunctionParamsParamTypeShared),
            "&" => symbols.push(Symbol::FunctionParamsParamTypeBorrowed),
            "," => {
              symbols.push(Symbol::FunctionParamsComma);
              break;
            }
            _ if is_whitespace(literal) => {
              symbolize_whitespace(&mut tokens, &mut symbols, literal)?
            }
            _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
          }
        }
      }
      ")" => {
        symbols.push(Symbol::FunctionParamsCloseParenthesis);

        loop {
          let token = tokens
            .pop()
            .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
          let literal = token.get_literal().as_str();

          match literal {
            ":" => {
              symbols.push(Symbol::FunctionReturnTypeColon);

              loop {
                let token = tokens
                  .pop()
                  .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
                let literal = token.get_literal().as_str();

                match literal {
                  name if is_identifier_name(name) => {
                    symbols.push(Symbol::FunctionParamsParamTypeName(name.to_string()))
                  }
                  "mutable" => symbols.push(Symbol::FunctionParamsParamTypeMutable),
                  "shared" => symbols.push(Symbol::FunctionParamsParamTypeShared),
                  "&" => symbols.push(Symbol::FunctionParamsParamTypeBorrowed),
                  "{" => {
                    tokens.push(token);
                    break;
                  }
                  _ if is_whitespace(literal) => {
                    symbolize_whitespace(&mut tokens, &mut symbols, literal)?
                  }
                  _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
                }
              }
            }
            "{" => {
              tokens.push(token);
              break;
            }
            _ if is_whitespace(literal) => {
              symbolize_whitespace(&mut tokens, &mut symbols, literal)?
            }
            _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
          }
        }

        break;
      }
      _ if is_whitespace(literal) => symbolize_whitespace(&mut tokens, &mut symbols, literal)?,
      _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
    }
  }

  return Ok(());
}

fn symbolize_function_body(
  mut tokens: &mut Vec<Token>,
  mut symbols: &mut Vec<Symbol>,
  _literal: &str,
) -> Result<(), Error> {
  symbols.push(Symbol::FunctionExpressionsOpenBrace);

  loop {
    let token = tokens
      .pop()
      .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
    let literal = token.get_literal().as_str();

    match literal {
      "\"" => symbolize_string(&mut tokens, &mut symbols, literal)?,
      "&" => symbols.push(Symbol::InstanceBorrow),
      "let" | "const" => {
        match literal {
          "let" => symbols.push(Symbol::Let),
          _ => symbols.push(Symbol::Const),
        };

        let mut token = tokens
          .pop()
          .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
        let mut literal = token.get_literal().as_str();

        if is_whitespace(literal) {
          symbolize_whitespace(&mut tokens, &mut symbols, literal)?;

          token = tokens
            .pop()
            .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
          literal = token.get_literal().as_str();
        }

        match literal {
          name if is_identifier_name(name) => symbols.push(Symbol::VariableName(name.to_string())),
          _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
        }

        token = tokens
          .pop()
          .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
        literal = token.get_literal().as_str();

        if is_whitespace(literal) {
          symbolize_whitespace(&mut tokens, &mut symbols, literal)?;

          token = tokens
            .pop()
            .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
          literal = token.get_literal().as_str();
        }

        match literal {
          "=" => symbols.push(Symbol::Assignment),
          _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
        }
      }
      ")" => symbols.push(Symbol::FunctionCallCloseParenthesis),
      ";" => symbols.push(Symbol::Semicolon),
      "=" => symbols.push(Symbol::Assignment),
      name if is_identifier_name(name) => {
        let mut token_peek = tokens
          .last()
          .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
        let mut peek = token_peek.get_literal().as_str();

        while is_whitespace(peek) {
          tokens.pop();

          token_peek = tokens
            .last()
            .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
          peek = token_peek.get_literal().as_str();
        }

        match peek {
          "::" => {
            symbols.push(Symbol::TypeAccessName(name.to_string()));

            symbols.push(Symbol::TypeAccessDoubleSemicolon);
            tokens.pop();
          }
          "." => {
            symbols.push(Symbol::InstanceAccessName(name.to_string()));

            symbols.push(Symbol::InstanceAccessPeriod);
            tokens.pop();
          }
          "(" => {
            symbols.push(Symbol::FunctionCallName(name.to_string()));

            symbols.push(Symbol::FunctionCallOpenParenthesis);
            tokens.pop();
          }
          _ if is_whitespace(literal) => symbolize_whitespace(&mut tokens, &mut symbols, literal)?,
          _ => symbols.push(Symbol::InstanceReferenceName(String::from(name))),
        }
      }
      "}" => {
        symbols.push(Symbol::FunctionExpressionsCloseBrace);
        break;
      }
      _ if is_whitespace(literal) => symbolize_whitespace(&mut tokens, &mut symbols, literal)?,
      _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
    }
  }

  return Ok(());
}

fn symbolize_whitespace(
  tokens: &mut Vec<Token>,
  _symbols: &mut Vec<Symbol>,
  _literal: &str,
) -> Result<(), Error> {
  // symbols.push(Symbol::Whitespace);

  while let Some(token) = tokens.last() {
    let literal = token.get_literal().as_str();

    if !is_whitespace(literal) {
      break;
    }

    tokens.pop();
  }

  return Ok(());
}

fn expect_next(
  mut tokens: &mut Vec<Token>,
  mut symbols: &mut Vec<Symbol>,
  expected: &dyn Fn(&str) -> bool,
  allow_whitespace: bool,
) -> Result<(), Error> {
  let mut token = tokens
    .pop()
    .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
  let mut literal = token.get_literal().as_str();

  if allow_whitespace && is_whitespace(literal) {
    symbolize_whitespace(&mut tokens, &mut symbols, literal)?;

    token = tokens
      .pop()
      .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
    literal = token.get_literal().as_str();
  }

  if !expected(literal) {
    todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols);
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
