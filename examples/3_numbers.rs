mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::Console;

fn main() {
    let x: isize = 2;
    let y: isize = 3;

    Console::write_line(LangString::from_owned((x + y).to_string()));
    Console::write_line(LangString::from_owned((x - y).to_string()));
    Console::write_line(LangString::from_owned((x * y).to_string()));
    Console::write_line(LangString::from_owned((x as f64 / y as f64).to_string()));
    Console::write_line(LangString::from_owned(x.pow(y as u32).to_string()));
}
