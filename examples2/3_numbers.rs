#![feature(const_fn_trait_bound)]

mod fe_prelude;
mod fe_std;

use fe_prelude::*;
use fe_std::Console;

fn main() {
    let x: isize = 2;
    let y: isize = 3;

    Console::write_line(FeString::from_owned((x + y).to_string()));
    Console::write_line(FeString::from_owned((x - y).to_string()));
    Console::write_line(FeString::from_owned((x * y).to_string()));
    Console::write_line(FeString::from_owned((x as f64 / y as f64).to_string()));
    Console::write_line(FeString::from_owned(x.pow(y as u32).to_string()));
}
