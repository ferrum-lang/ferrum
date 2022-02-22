mod fe_prelude;
mod fe_std;

use fe_prelude::*;
use fe_std::{Console, FeStringBuilder};

#[allow(non_upper_case_globals)]
const STR_SLICE_0: FeString = FeString::from_slice("Fizz");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: FeString = FeString::from_slice("Buzz");

fn fizzbuzz(n: usize) -> FeString {
    let is_fizz = n % 3 == 0;
    let is_buzz = n % 5 == 0;

    if !is_fizz && !is_buzz {
        return FeString::from_owned(n.to_string());
    }

    let mut builder = FeStringBuilder::new();

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
