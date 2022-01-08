mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{ Console, LangStringBuilder, };

#[allow(non_upper_case_globals)]
const SLICE_Fizz: LangString = LangString::from_slice("Fizz");

#[allow(non_upper_case_globals)]
const SLICE_Buzz: LangString = LangString::from_slice("Buzz");

fn fizzbuzz(n: usize) -> LangString {
  let is_fizz = n % 3 == 0;
  let is_buzz = n % 5 == 0;

  if !is_fizz && !is_buzz {
    return LangString::from_owned(n.to_string());
  }

  let mut builder = LangStringBuilder::new();

  if is_fizz {
    builder.append(SLICE_Fizz);
  }

  if is_buzz {
    builder.append(SLICE_Buzz);
  }

  return builder.build();
}

fn main() {
  for n in 1..=20 {
    Console::write_line(fizzbuzz(n));
  }
}
