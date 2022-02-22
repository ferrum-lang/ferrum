mod fe_prelude;
mod fe_std;

use fe_prelude::*;
use fe_std::Console;

#[allow(non_upper_case_globals)]
const STR_SLICE_0: FeString = FeString::from_slice("Hello world");

fn main() {
    Console::write_line(STR_SLICE_0);
}
