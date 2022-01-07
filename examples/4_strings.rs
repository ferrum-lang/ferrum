mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{ Console, LangStringBuilder, };

fn main() {

  let x = LangString::from_slice("abc");
  Console::write_line(x);

  let y = 123;

  let x = LangString::from_owned(format!("abc {}", y));
  Console::write_line(x);

  let x = LangString::from_slice("abc {y}");
  Console::write_line(x);

  let x = LangString::from_owned(format!("abc \\{}", y));
  Console::write_line(x);

  let mut x = LangStringBuilder::new();
  x.append(LangString::from_slice(" "));
  x.prepend(LangString::from_slice("abc"));
  x.append(LangString::from_owned(y.to_string()));
  let x = x.build();

  let x = LangStringBuilder::from(LangString::from_slice(" "))
    .with_prepend(LangString::from_slice("abc"))
    .with_append(LangString::from_owned(y.to_string()))
    .build();

  let z: Vec<char> = x.as_slice().chars().collect();
  let z: std::str::Chars = x.as_slice().chars();

  let z: Vec<u8> = x.as_slice().bytes().collect();
  let z: std::str::Bytes = x.as_slice().bytes();

  let z: usize = x.as_slice().len();
}