mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{ Console, LangStringBuilder, };

fn main() {
  let x: (isize, isize, isize) = (1, 2, 3);
  let x: (isize, LangString) = (42, LangString::from_slice("Adam"));
  
  Console::write_line(LangString::from_owned(format!("{} - {}", x.0, x.1)));
  
  let x = (
    (0),
    (1, 2, 3),
    (4, 5, 6),
    (7, 8, 9),
    (10),
  );
  
  let (a, b) = (LangString::from_slice("abc"), 123);
  
  Console::write_line(LangString::from_owned(format!("{}, {}", a, b)));
  
  Console::write_line(
    LangStringBuilder::new()
      .with_append(LangString::from_owned(1.to_string()))
      .with_append(LangString::from_slice(", "))
      .with_append(LangString::from_slice("two"))
      .with_append(LangString::from_slice(", "))
      .with_append(LangString::from_owned(3.to_string()))
      .build()
  );

  Console::write_line(
    LangStringBuilder::new()
      .with_append(LangString::from_owned(1.to_string()))
      .with_append(LangString::from_slice("_"))
      .with_append(LangString::from_owned(2.to_string()))
      .with_append(LangString::from_slice("_"))
      .with_append(LangString::from_owned(3.to_string()))
      .build()
  );
  
  let (a, b): (isize, LangString) = (1, LangString::from_slice("two"));
  let rest: (isize, LangString, isize) = (3, LangString::from_slice("four"), 5);
  let (end): (LangString) = (LangString::from_slice("six"));
  
  Console::write_line(LangString::from_owned(format!("{}, {}", a, b)));
  Console::write_line(
    LangStringBuilder::new()
      .with_append(LangString::from_owned(rest.0.to_string()))
      .with_append(LangString::from_slice(", "))
      .with_append(rest.1)
      .with_append(LangString::from_slice(", "))
      .with_append(LangString::from_owned(rest.2.to_string()))
      .build()
  );
  Console::write_line(end);
}
