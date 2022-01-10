mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{Console, LangStringBuilder};

#[allow(non_upper_case_globals)]
const STR_SLICE_0: LangString = LangString::from_slice("Fizz");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: LangString = LangString::from_slice("Buzz");

fn fizzbuzz(n: usize) -> LangString {
    let is_fizz = n % 3 == 0;
    let is_buzz = n % 5 == 0;

    if !is_fizz && !is_buzz {
        return LangString::from_owned(n.to_string());
    }

    let mut builder = LangStringBuilder::new();

    if is_fizz {
        builder.append(STR_SLICE_0);
    }

    if is_buzz {
        builder.append(STR_SLICE_1);
    }

    return builder.build();
}

fn main() {
    for n in 1..=20 {
        Console::write_line(fizzbuzz(n));
    }
}
