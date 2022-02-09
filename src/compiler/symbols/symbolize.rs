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
      _ if is_comment(literal) => {}
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
        symbolize_function_param(&mut tokens, &mut symbols, name)?;
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
              symbolize_function_return(&mut tokens, &mut symbols, literal)?;
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

fn symbolize_function_param(
  mut tokens: &mut Vec<Token>,
  mut symbols: &mut Vec<Symbol>,
  literal: &str,
) -> Result<(), Error> {
  let mut token;
  let mut literal = literal;

  loop {
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

        break;
      }
      _ if is_whitespace(literal) => symbolize_whitespace(&mut tokens, &mut symbols, literal)?,
      _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
    }

    token = tokens
      .pop()
      .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
    literal = token.get_literal().as_str();
  }

  return Ok(());
}

fn symbolize_function_return(
  mut tokens: &mut Vec<Token>,
  mut symbols: &mut Vec<Symbol>,
  literal: &str,
) -> Result<(), Error> {
  let mut token;
  let mut literal = literal;

  loop {
    match literal {
      ":" => {
        symbols.push(Symbol::FunctionReturnTypeColon);

        let token = tokens
          .pop()
          .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
        let literal = token.get_literal().as_str();

        symbolize_type(&mut tokens, &mut symbols, literal)?;

        break;
      }
      _ if is_whitespace(literal) => symbolize_whitespace(&mut tokens, &mut symbols, literal)?,
      _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
    }

    token = tokens
      .pop()
      .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
    literal = token.get_literal().as_str();
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

        if literal == ":" {
          symbols.push(Symbol::VariableTypeColon);

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

          symbolize_type(&mut tokens, &mut symbols, literal)?;

          let literal = expect_next(
            &mut tokens,
            &mut symbols,
            &|literal| literal == "=" || literal == ";",
            true,
          )?;
          tokens.push(Token::new(literal));
        } else {
          tokens.push(Token::new(literal));
        }
      }
      ")" => symbols.push(Symbol::FunctionCallCloseParenthesis),
      "." => symbols.push(Symbol::InstanceAccessPeriod),
      ";" => symbols.push(Symbol::Semicolon),
      "=" => symbols.push(Symbol::Assignment),
      "}" => {
        symbols.push(Symbol::FunctionExpressionsCloseBrace);
        break;
      }
      "\"" | "'" | "&" | "[" | "(" => symbolize_expression(&mut tokens, &mut symbols, literal)?,
      number if is_numeric(number) => symbolize_expression(&mut tokens, &mut symbols, number)?,
      name if is_identifier_name(name) => symbolize_expression(&mut tokens, &mut symbols, name)?,
      _ if is_whitespace(literal) => symbolize_whitespace(&mut tokens, &mut symbols, literal)?,
      _ if is_comment(literal) => {}
      _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
    }
  }

  return Ok(());
}

fn symbolize_expression(
  mut tokens: &mut Vec<Token>,
  mut symbols: &mut Vec<Symbol>,
  literal: &str,
) -> Result<(), Error> {
  let literal_string = String::from(literal);

  let mut token;
  let mut literal = literal_string.as_str();

  loop {
    match literal {
      "\"" => {
        symbolize_string(&mut tokens, &mut symbols, literal)?;

        break;
      }
      "'" => {
        let token = tokens
          .pop()
          .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));

        symbols.push(Symbol::Char(token.take_literal()));

        expect_next(&mut tokens, &mut symbols, &|literal| literal == "'", false)?;

        break;
      }
      "true" => {
        symbols.push(Symbol::True);
        break;
      }
      "false" => {
        symbols.push(Symbol::False);
        break;
      }
      number if is_numeric(number) => {
        // TODO: Handle floats + more
        symbols.push(Symbol::Int(String::from(number)));

        break;
      }
      name if name == "&" || is_identifier_name(name) => {
        let name_string = if literal == "&" {
          symbols.push(Symbol::InstanceBorrow);

          let token = tokens
            .pop()
            .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
          token.take_literal()
        } else {
          String::from(name)
        };
        let name = name_string.as_str();

        let mut token_peek = tokens
          .last()
          .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
        let mut peek = token_peek.get_literal().as_str();

        while is_whitespace(peek) {
          tokens.pop();

          token_peek = tokens
            .last()
            .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
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

        break;
      }
      "(" => {
        if is_closure(&mut tokens, &mut symbols, "(") {
          symbols.push(Symbol::ClosureParamsOpen);

          loop {
            let token = tokens
              .pop()
              .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
            let literal = token.get_literal().as_str();

            match literal {
              "," => {
                symbols.push(Symbol::ClosureParamsComma);
              }
              ")" => {
                symbols.push(Symbol::ClosureParamsClose);
                break;
              }
              _ => {
                symbolize_function_param(&mut tokens, &mut symbols, literal)?;
              }
            }
          }

          let mut token = tokens
            .pop()
            .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
          let mut literal = token.get_literal().as_str();

          if is_whitespace(literal) {
            symbolize_whitespace(&mut tokens, &mut symbols, literal)?;
            token = tokens
              .pop()
              .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
            literal = token.get_literal().as_str();
          }

          if literal == ":" {
            symbolize_function_return(&mut tokens, &mut symbols, literal)?;
          } else {
            tokens.push(token);
          }

          expect_next(&mut tokens, &mut symbols, &|literal| literal == "=>", true)?;
          symbols.push(Symbol::ClosureArrow);

          token = tokens
            .pop()
            .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
          literal = token.get_literal().as_str();

          if is_whitespace(literal) {
            symbolize_whitespace(&mut tokens, &mut symbols, literal)?;
            token = tokens
              .pop()
              .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
            literal = token.get_literal().as_str();
          }

          match literal {
            "{" => symbolize_function_body(&mut tokens, &mut symbols, literal)?,
            _ => symbolize_expression(&mut tokens, &mut symbols, literal)?,
          }
        } else {
          symbols.push(Symbol::TupleStart);

          loop {
            let token = tokens
              .pop()
              .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
            let literal = token.get_literal().as_str();

            match literal {
              "," => {
                symbols.push(Symbol::TupleComma);
              }
              ";" => {
                symbols.push(Symbol::TupleSemicolon);

                let literal = expect_next(
                  &mut tokens,
                  &mut symbols,
                  &|literal| is_numeric(literal),
                  true,
                )?;
                symbols.push(Symbol::TupleLength(literal));

                expect_next(&mut tokens, &mut symbols, &|literal| literal == ")", true)?;
                symbols.push(Symbol::TupleEnd);
                break;
              }
              ")" => {
                symbols.push(Symbol::TupleEnd);
                break;
              }
              _ => {
                symbolize_expression(&mut tokens, &mut symbols, literal)?;
              }
            }
          }
        }

        break;
      }
      "[" => {
        symbols.push(Symbol::ListOpen);

        loop {
          let token = tokens
            .pop()
            .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
          let literal = token.get_literal().as_str();
          match literal {
            "," => {
              symbols.push(Symbol::ListComma);
            }
            "]" => {
              symbols.push(Symbol::ListClose);
              break;
            }
            _ => {
              symbolize_expression(&mut tokens, &mut symbols, literal)?;
            }
          }
        }

        break;
      }
      _ if is_whitespace(literal) => symbolize_whitespace(&mut tokens, &mut symbols, literal)?,
      _ if is_comment(literal) => {}
      _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
    }

    token = tokens
      .pop()
      .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
    literal = token.get_literal().as_str();
  }

  token = tokens
    .pop()
    .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
  literal = token.get_literal().as_str();

  if is_whitespace(literal) {
    symbolize_whitespace(&mut tokens, &mut symbols, literal)?;
    token = tokens
      .pop()
      .expect(&format!("Unfinished function!\n\nSymbols: {:?}", symbols));
    literal = token.get_literal().as_str();
  }

  match literal {
    "+" => {
      symbols.push(Symbol::Plus);

      token = tokens
        .pop()
        .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
      literal = token.get_literal().as_str();

      symbolize_expression(&mut tokens, &mut symbols, literal)?;
    }
    "-" => {
      symbols.push(Symbol::Minus);

      token = tokens
        .pop()
        .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
      literal = token.get_literal().as_str();

      symbolize_expression(&mut tokens, &mut symbols, literal)?;
    }
    "*" => {
      symbols.push(Symbol::Multiply);

      token = tokens
        .pop()
        .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
      literal = token.get_literal().as_str();

      symbolize_expression(&mut tokens, &mut symbols, literal)?;
    }
    "/" => {
      symbols.push(Symbol::Divide);

      token = tokens
        .pop()
        .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
      literal = token.get_literal().as_str();

      symbolize_expression(&mut tokens, &mut symbols, literal)?;
    }
    "^" => {
      symbols.push(Symbol::Exponent);

      token = tokens
        .pop()
        .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
      literal = token.get_literal().as_str();

      symbolize_expression(&mut tokens, &mut symbols, literal)?;
    }
    ".." => {
      // TODO: Look into `x..` iterating with no defined end
      symbols.push(Symbol::Range);

      token = tokens
        .pop()
        .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
      literal = token.get_literal().as_str();

      symbolize_expression(&mut tokens, &mut symbols, literal)?;
    }
    "?" => {
      if is_ternary(&mut tokens, &mut symbols, literal) {
        symbols.push(Symbol::TernaryQuestion);

        token = tokens
          .pop()
          .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
        literal = token.get_literal().as_str();

        symbolize_expression(&mut tokens, &mut symbols, literal)?;

        expect_next(&mut tokens, &mut symbols, &|literal| literal == ":", true)?;
        symbols.push(Symbol::TernaryOr);

        token = tokens
          .pop()
          .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
        literal = token.get_literal().as_str();

        symbolize_expression(&mut tokens, &mut symbols, literal)?;
      } else {
        symbols.push(Symbol::CastQuestion);
      }
    }
    "??" => {
      symbols.push(Symbol::NullCoalesce);

      token = tokens
        .pop()
        .expect(&format!("Unfinished expression!\n\nSymbols: {:?}", symbols));
      literal = token.get_literal().as_str();

      symbolize_expression(&mut tokens, &mut symbols, literal)?;
    }
    _ => {
      tokens.push(token);
    }
  }

  return Ok(());
}

fn symbolize_type(
  mut tokens: &mut Vec<Token>,
  mut symbols: &mut Vec<Symbol>,
  literal: &str,
) -> Result<(), Error> {
  let token;
  let mut literal = literal;

  if is_whitespace(literal) {
    symbolize_whitespace(&mut tokens, &mut symbols, literal)?;
    token = tokens.pop().expect(&format!(
      "Unfinished variable type!\n\nSymbols: {:?}",
      symbols
    ));
    literal = token.get_literal().as_str();
  }

  match literal {
    "(" => {
      symbols.push(Symbol::TupleTypeStart);

      loop {
        let token = tokens
          .pop()
          .expect(&format!("Unfinished type!\n\nSymbols: {:?}", symbols));
        let literal = token.get_literal().as_str();

        match literal {
          "," => {
            symbols.push(Symbol::TupleTypeComma);
          }
          ";" => {
            symbols.push(Symbol::TupleTypeSemicolon);

            let literal = expect_next(
              &mut tokens,
              &mut symbols,
              &|literal| is_numeric(literal),
              true,
            )?;

            symbols.push(Symbol::TupleTypeLength(literal));

            expect_next(&mut tokens, &mut symbols, &|literal| literal == ")", true)?;
            symbols.push(Symbol::TupleTypeEnd);
            break;
          }
          ")" => {
            symbols.push(Symbol::TupleTypeEnd);
            break;
          }
          _ => {
            symbolize_type(&mut tokens, &mut symbols, literal)?;
          }
        }
      }
    }
    "boolean" | "uint" | "uint1" | "uint8" | "uint16" | "uint32" | "uint64" | "uint128"
    | "uint256" | "biguint" | "bit" | "byte" | "int" | "int8" | "int16" | "int32" | "int64"
    | "int128" | "int256" | "bigint" | "float32" | "float64" => {
      symbols.push(Symbol::TypeName(String::from(literal)));

      let token = tokens
        .pop()
        .expect(&format!("Unfinished type!\n\nSymbols: {:?}", symbols));
      let literal = token.get_literal().as_str();

      if literal == "?" {
        symbols.push(Symbol::TypeOptional);
      } else {
        tokens.push(Token::new(String::from(literal)));
      }
    }
    _ if is_identifier_name(literal) => {
      symbols.push(Symbol::TypeName(String::from(literal)));

      let mut token = tokens
        .pop()
        .expect(&format!("Unfinished type!\n\nSymbols: {:?}", symbols));
      let mut literal = token.get_literal().as_str();

      match literal {
        "<" => {
          symbols.push(Symbol::TypeGenericOpen);

          token = tokens
            .pop()
            .expect(&format!("Unfinished type!\n\nSymbols: {:?}", symbols));
          literal = token.get_literal().as_str();

          symbolize_type(&mut tokens, &mut symbols, literal)?;

          loop {
            token = tokens
              .pop()
              .expect(&format!("Unfinished type!\n\nSymbols: {:?}", symbols));
            literal = token.get_literal().as_str();

            match literal {
              "," => {
                symbols.push(Symbol::TypeGenericComma);

                token = tokens
                  .pop()
                  .expect(&format!("Unfinished type!\n\nSymbols: {:?}", symbols));
                literal = token.get_literal().as_str();

                symbolize_type(&mut tokens, &mut symbols, literal)?;
              }
              ">" => {
                symbols.push(Symbol::TypeGenericClose);
                break;
              }
              _ if is_whitespace(literal) => {}
              _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
            }
          }
        }
        "?" => {
          symbols.push(Symbol::TypeOptional);
        }
        "::" => {
          symbols.push(Symbol::TypeAccessColons);

          token = tokens
            .pop()
            .expect(&format!("Unfinished type!\n\nSymbols: {:?}", symbols));
          literal = token.get_literal().as_str();

          symbolize_type(&mut tokens, &mut symbols, literal)?;
        }
        _ => {
          tokens.push(Token::new(String::from(literal)));
        }
      }
    }
    _ if is_whitespace(literal) => symbolize_whitespace(&mut tokens, &mut symbols, literal)?,
    _ => todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols),
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
) -> Result<String, Error> {
  let mut token = tokens
    .pop()
    .expect(&format!("Unfinished!\n\nSymbols: {:?}", symbols));
  let mut literal = token.get_literal().as_str();

  if allow_whitespace && is_whitespace(literal) {
    symbolize_whitespace(&mut tokens, &mut symbols, literal)?;

    token = tokens
      .pop()
      .expect(&format!("Unfinished!\n\nSymbols: {:?}", symbols));
    literal = token.get_literal().as_str();
  }

  if !expected(literal) {
    todo!("Unexpected token: {}\n\nSymbols: {:?}", literal, symbols);
  }

  return Ok(String::from(literal));
}

fn is_whitespace(string: &str) -> bool {
  string.trim().is_empty()
}

fn is_comment(string: &str) -> bool {
  return string.starts_with("//") || string.starts_with("/*");
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

fn is_closure(tokens: &mut Vec<Token>, symbols: &mut Vec<Symbol>, _literal: &str) -> bool {
  let mut history = vec![];
  let mut parenthesis_count = 1;

  let mut is_closure = None;

  while is_closure.is_none() && parenthesis_count > 0 {
    let token = tokens
      .pop()
      .expect(&format!("Unfinished!\n\nSymbols: {:?}", symbols));
    let literal = token.get_literal().as_str();

    match literal {
      "(" => parenthesis_count += 1,
      ")" => parenthesis_count -= 1,
      _ => {}
    }

    if literal == ":" {
      // Tuples don't use `:`
      is_closure = Some(true);
    }

    history.push(token);
  }

  while is_closure.is_none() {
    let token = tokens
      .pop()
      .expect(&format!("Unfinished!\n\nSymbols: {:?}", symbols));
    let literal = token.get_literal().as_str();

    // Closures will either have `:` or `=>` immediately after the params, ignoring whitespace
    match literal {
      ":" | "=>" => {
        is_closure = Some(true);
      }
      _ if is_whitespace(literal) => {
        // Ignore whitespace
      }
      _ => {
        is_closure = Some(false);
      }
    }

    history.push(token);
  }

  while let Some(item) = history.pop() {
    tokens.push(item);
  }

  return is_closure.unwrap_or(false);
}

fn is_ternary(tokens: &mut Vec<Token>, symbols: &mut Vec<Symbol>, _literal: &str) -> bool {
  let mut history = vec![];

  let mut is_ternary = None;

  while is_ternary.is_none() {
    let token = tokens
      .pop()
      .expect(&format!("Unfinished!\n\nSymbols: {:?}", symbols));
    let literal = token.get_literal().as_str();

    // TODO: make it easier to understand what is an "expression-ending" character
    match literal {
      ")" | "]" | "," | "." | "+" | "-" | "*" | "/" | "%" | "^" | ";" | "|" | "||" | "&&" | ":" => {
        is_ternary = Some(false);
      }
      _ if is_whitespace(literal) => {}
      _ => {
        is_ternary = Some(true);
      }
    }
  }

  while let Some(item) = history.pop() {
    tokens.push(item);
  }

  return is_ternary.unwrap_or(false);
}
