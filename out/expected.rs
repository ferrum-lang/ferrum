mod lang_prelude;
use lang_prelude::*;

mod lang_std;
use lang_std::{ Console, };

fn main() {
  Console::write_line(LangString::from_slice("Hello world"));
}
