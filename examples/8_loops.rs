mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{ Console, };

fn main() {
  let mut i = 0;
  loop {
    if i >= 10 {
      break;
    }

    Console::write_line(LangString::from_owned(format!("loop: {}", i)));

    i += 1;
  }

  let mut i = 0;
  while i < 10 {
    Console::write_line(LangString::from_owned(format!("while: {}", i)));
    i += 1;
  }

  for i in 0..10 {
    Console::write_line(LangString::from_owned(format!("for excl: {}", i)));
  }

  for i in 0..=10 {
    Console::write_line(LangString::from_owned(format!("for incl: {}", i)));
  }

  let x = vec![1, 2, 3];

  for n in x.iter() {
    Console::write_line(LangString::from_owned(format!("for in arr: {}", n)));
  }

  for (idx, n) in x.iter().enumerate() {
    Console::write_line(LangString::from_owned(format!("for with idx: ({}, {})", idx, n)));
  }
}
