mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{ Console, };

fn main() {
  if true {
    Console::write_line(LangString::from_slice("true"));
  } else if false {
    Console::write_line(LangString::from_slice("false"));
  } else {
    Console::write_line(LangString::from_slice("else"));
  }

  let x = 1;
  let y = 2;
  let z = LangString::from_slice("Adam");

  if (x == 0 || y == 0) && z.as_slice().len() > 0 {
    Console::write_line(LangString::from_owned(1.to_string()));
  }

  let x = Some(123);

  if let Some(y) = x {
    Console::write_line(LangString::from_owned(y.to_string()));
  }

  match x {
    None => Console::write_line(LangString::from_owned(2.to_string())),
    _ => Console::write_line(LangString::from_owned(3.to_string())),
  }

  match "Adam" {
    "Bates" => Console::write_line(LangString::from_owned(4.to_string())),
    "" => Console::write_line(LangString::from_owned(5.to_string())),
    x if x.len() > 3 => Console::write_line(LangString::from_owned(format!("{}: 6", x))),
    _ => Console::write_line(LangString::from_owned(7.to_string())),
  }
}