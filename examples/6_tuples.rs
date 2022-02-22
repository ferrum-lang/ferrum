mod fe_prelude;
mod fe_std;

use fe_prelude::*;
use fe_std::{Console, FeStringBuilder};

#[allow(non_upper_case_globals)]
const STR_SLICE_0: FeString = FeString::from_slice("Adam");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: FeString = FeString::from_slice("abc");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: FeString = FeString::from_slice("1");

#[allow(non_upper_case_globals)]
const STR_SLICE_3: FeString = FeString::from_slice(", ");

#[allow(non_upper_case_globals)]
const STR_SLICE_4: FeString = FeString::from_slice("two");

#[allow(non_upper_case_globals)]
const STR_SLICE_5: FeString = FeString::from_slice("3");

#[allow(non_upper_case_globals)]
const STR_SLICE_6: FeString = FeString::from_slice("_");

#[allow(non_upper_case_globals)]
const STR_SLICE_7: FeString = FeString::from_slice("2");

#[allow(non_upper_case_globals)]
const STR_SLICE_8: FeString = FeString::from_slice("four");

#[allow(non_upper_case_globals)]
const STR_SLICE_9: FeString = FeString::from_slice("six");

fn main() {
    let x: (isize, isize, isize) = (1, 2, 3);
    let x: (isize, FeString) = (42, STR_SLICE_0);

    Console::write_line(FeString::from_owned(format!("{} - {}", x.0, x.1)));

    let x = ((0), (1, 2, 3), (4, 5, 6), (7, 8, 9), (10));

    let (a, b) = (STR_SLICE_1, 123);

    Console::write_line(FeString::from_owned(format!("{}, {}", a, b)));

    Console::write_line(
        FeStringBuilder::new()
            .with_append(STR_SLICE_2)
            .with_append(STR_SLICE_3)
            .with_append(STR_SLICE_4)
            .with_append(STR_SLICE_3)
            .with_append(STR_SLICE_5)
            .build(),
    );

    Console::write_line(
        FeStringBuilder::new()
            .with_append(STR_SLICE_2)
            .with_append(STR_SLICE_6)
            .with_append(STR_SLICE_7)
            .with_append(STR_SLICE_6)
            .with_append(STR_SLICE_5)
            .build(),
    );

    let (a, b): (isize, FeString) = (1, STR_SLICE_4);
    let rest: (isize, FeString, isize) = (3, STR_SLICE_8, 5);
    let (end): (FeString) = (STR_SLICE_9);

    Console::write_line(FeString::from_owned(format!("{}, {}", a, b)));
    Console::write_line(
        FeStringBuilder::new()
            .with_append(FeString::from_owned(rest.0.to_string()))
            .with_append(STR_SLICE_3)
            .with_append(rest.1)
            .with_append(STR_SLICE_3)
            .with_append(FeString::from_owned(rest.2.to_string()))
            .build(),
    );
    Console::write_line(end);
}
