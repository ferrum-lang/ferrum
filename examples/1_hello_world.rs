mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::Console;

#[allow(non_upper_case_globals)]
const STR_SLICE_0: LangString = LangString::from_slice("Hello world");

fn main() {
    Console::write_line(STR_SLICE_0);
}
