#![feature(const_fn_trait_bound)]

mod fe_prelude;
mod fe_std;

use fe_prelude::*;
use fe_std::{Console, FeStringBuilder};

#[allow(non_upper_case_globals)]
const STR_SLICE_0: FeString = FeString::from_slice("abc");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: FeString = FeString::from_slice("abc {y}");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: FeString = FeString::from_slice(" ");

fn main() {
    let x = STR_SLICE_0;
    Console::write_line(x);

    let y: isize = 123;

    let x = FeString::from_owned(format!("abc {}", y));
    Console::write_line(x);

    let x = STR_SLICE_1;
    Console::write_line(x);

    let x = FeString::from_owned(format!("abc \\{}", y));
    Console::write_line(x);

    let mut x = FeStringBuilder::new();
    x.append(STR_SLICE_2);
    x.prepend(STR_SLICE_0);
    x.append(FeString::from_owned(y.to_string()));
    let x = x.build();

    let x = FeStringBuilder::from(STR_SLICE_2)
        .with_prepend(STR_SLICE_0)
        .with_append(FeString::from_owned(y.to_string()))
        .build();

    let z: Vec<char> = x.as_slice().chars().collect();
    let z: std::str::Chars = x.as_slice().chars();

    let z: Vec<u8> = x.as_slice().bytes().collect();
    let z: std::str::Bytes = x.as_slice().bytes();

    let z: usize = x.as_slice().len();
}
