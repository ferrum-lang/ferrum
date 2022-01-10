mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{Console, LangStringBuilder};

#[allow(non_upper_case_globals)]
const STR_SLICE_0: LangString = LangString::from_slice("Adam");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: LangString = LangString::from_slice("abc");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: LangString = LangString::from_slice("1");

#[allow(non_upper_case_globals)]
const STR_SLICE_3: LangString = LangString::from_slice(", ");

#[allow(non_upper_case_globals)]
const STR_SLICE_4: LangString = LangString::from_slice("two");

#[allow(non_upper_case_globals)]
const STR_SLICE_5: LangString = LangString::from_slice("3");

#[allow(non_upper_case_globals)]
const STR_SLICE_6: LangString = LangString::from_slice("_");

#[allow(non_upper_case_globals)]
const STR_SLICE_7: LangString = LangString::from_slice("2");

#[allow(non_upper_case_globals)]
const STR_SLICE_8: LangString = LangString::from_slice("four");

#[allow(non_upper_case_globals)]
const STR_SLICE_9: LangString = LangString::from_slice("six");

fn main() {
    let x: (isize, isize, isize) = (1, 2, 3);
    let x: (isize, LangString) = (42, STR_SLICE_0);

    Console::write_line(LangString::from_owned(format!("{} - {}", x.0, x.1)));

    let x = ((0), (1, 2, 3), (4, 5, 6), (7, 8, 9), (10));

    let (a, b) = (STR_SLICE_1, 123);

    Console::write_line(LangString::from_owned(format!("{}, {}", a, b)));

    Console::write_line(
        LangStringBuilder::new()
            .with_append(STR_SLICE_2)
            .with_append(STR_SLICE_3)
            .with_append(STR_SLICE_4)
            .with_append(STR_SLICE_3)
            .with_append(STR_SLICE_5)
            .build(),
    );

    Console::write_line(
        LangStringBuilder::new()
            .with_append(STR_SLICE_2)
            .with_append(STR_SLICE_6)
            .with_append(STR_SLICE_7)
            .with_append(STR_SLICE_6)
            .with_append(STR_SLICE_5)
            .build(),
    );

    let (a, b): (isize, LangString) = (1, STR_SLICE_4);
    let rest: (isize, LangString, isize) = (3, STR_SLICE_8, 5);
    let (end): (LangString) = (STR_SLICE_9);

    Console::write_line(LangString::from_owned(format!("{}, {}", a, b)));
    Console::write_line(
        LangStringBuilder::new()
            .with_append(LangString::from_owned(rest.0.to_string()))
            .with_append(STR_SLICE_3)
            .with_append(rest.1)
            .with_append(STR_SLICE_3)
            .with_append(LangString::from_owned(rest.2.to_string()))
            .build(),
    );
    Console::write_line(end);
}
