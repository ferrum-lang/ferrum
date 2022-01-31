mod lang_prelude;
use lang_prelude::*;

mod lang_std;
use lang_std::Console;

fn say_hello(name: LangString) {
  Console::write_line(LangString::from_owned(format!("Hello, {}!", name)));
}

pub fn main() {
  let name = LangString::from_slice("Adam");
  say_hello(LangString::from_owned(format!("{} Bates", name)));
  say_hello(name);
}
