mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{Console, LangStringBuilder};

#[allow(non_upper_case_globals)]
const STR_SLICE_0: LangString = LangString::from_slice("abc");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: LangString = LangString::from_slice("abc {y}");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: LangString = LangString::from_slice(" ");

fn main() {
    let x = STR_SLICE_0;
    Console::write_line(x);

    let y = 123;

    let x = LangString::from_owned(format!("abc {}", y));
    Console::write_line(x);

    let x = STR_SLICE_1;
    Console::write_line(x);

    let x = LangString::from_owned(format!("abc \\{}", y));
    Console::write_line(x);

    let mut x = LangStringBuilder::new();
    x.append(STR_SLICE_2);
    x.prepend(STR_SLICE_0);
    x.append(LangString::from_owned(y.to_string()));
    let x = x.build();

    let x = LangStringBuilder::from(STR_SLICE_2)
        .with_prepend(STR_SLICE_0)
        .with_append(LangString::from_owned(y.to_string()))
        .build();

    let z: Vec<char> = x.as_slice().chars().collect();
    let z: std::str::Chars = x.as_slice().chars();

    let z: Vec<u8> = x.as_slice().bytes().collect();
    let z: std::str::Bytes = x.as_slice().bytes();

    let z: usize = x.as_slice().len();
}
