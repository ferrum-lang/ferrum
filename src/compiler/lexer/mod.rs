mod matchers;

use super::{tokens::UnparsedToken, Error};
use matchers::{get_token_matchers, TokenMatcher};

pub fn lex_into_tokens(contents: String) -> Result<Vec<UnparsedToken>, Error> {
  let token_matchers = get_token_matchers();

  let mut tokens = vec![];
  let mut iter = contents.chars().peekable();

  while let Some(c) = iter.next() {
    let mut is_matched = false;

    for token_matcher in token_matchers.iter() {
      // short-circuit
      if is_matched {
        break;
      }

      match token_matcher {
        TokenMatcher::SingleChar(c2) => {
          if c == *c2 {
            tokens.push(UnparsedToken::new(c));
            is_matched = true;
          }
        }
        TokenMatcher::SinglePredicate(predicate) => {
          if predicate(c) {
            tokens.push(UnparsedToken::new(c));
            is_matched = true;
          }
        }
        TokenMatcher::BufferedChar(c2) => {
          if c != *c2 {
            // check next matcher
            continue;
          }

          is_matched = true;

          let mut buffer = String::from(c);

          while let Some(&peek) = iter.peek() {
            if c != peek {
              break;
            }

            buffer.push(peek);
            iter.next();
          }

          tokens.push(UnparsedToken::new(buffer));
        }
        TokenMatcher::BufferedPredicate(predicate) => {
          let mut buffer = String::new();

          if !predicate(&buffer, c) {
            // check next matcher
            continue;
          }

          is_matched = true;

          buffer.push(c);

          while let Some(&peek) = iter.peek() {
            if !predicate(&buffer, peek) {
              break;
            }

            buffer.push(peek);
            iter.next();
          }

          tokens.push(UnparsedToken::new(buffer));
        }
      }
    }

    if !is_matched {
      todo!("Unexpected token: '{}'\nUnparsed tokens: {:?}\n", c, tokens);
    }
  }

  return Ok(tokens);
}
