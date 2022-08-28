#![feature(const_fn_trait_bound)]

mod fe_prelude;
mod fe_std;

use fe_prelude::*;
use fe_std::Console;

#[allow(non_upper_case_globals)]
const STR_SLICE_0: FeString = FeString::from_slice("true");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: FeString = FeString::from_slice("false");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: FeString = FeString::from_slice("else");

#[allow(non_upper_case_globals)]
const STR_SLICE_3: FeString = FeString::from_slice("Adam");

#[allow(non_upper_case_globals)]
const STR_SLICE_4: FeString = FeString::from_slice("1");

#[allow(non_upper_case_globals)]
const STR_SLICE_5: FeString = FeString::from_slice("2");

#[allow(non_upper_case_globals)]
const STR_SLICE_6: FeString = FeString::from_slice("3");

#[allow(non_upper_case_globals)]
const STR_SLICE_7: FeString = FeString::from_slice("4");

#[allow(non_upper_case_globals)]
const STR_SLICE_8: FeString = FeString::from_slice("5");

#[allow(non_upper_case_globals)]
const STR_SLICE_9: FeString = FeString::from_slice("7");

#[allow(non_upper_case_globals)]
const STR_SLICE_10: FeString = FeString::from_slice("yes");

#[allow(non_upper_case_globals)]
const STR_SLICE_11: FeString = FeString::from_slice("no");

#[allow(non_upper_case_globals)]
const STR_SLICE_12: FeString = FeString::from_slice("none");

#[allow(non_upper_case_globals)]
const STR_SLICE_13: FeString = FeString::from_slice("true");

#[allow(non_upper_case_globals)]
const STR_SLICE_14: FeString = FeString::from_slice("false");

fn main() {
    if true {
        Console::write_line(STR_SLICE_0);
    } else if false {
        Console::write_line(STR_SLICE_1);
    } else {
        Console::write_line(STR_SLICE_2);
    }

    let x = 1;
    let y = 2;
    let z = STR_SLICE_3;

    if (x == 0 || y == 0) && z.as_slice().len() > 0 {
        Console::write_line(STR_SLICE_4);
    }

    let x = Some(123);

    if let Some(y) = x {
        Console::write_line(FeString::from_owned(y.to_string()));
    }

    match x {
        None => Console::write_line(STR_SLICE_5),
        _ => Console::write_line(STR_SLICE_6),
    }

    match "Adam" {
        "Bates" => Console::write_line(STR_SLICE_7),
        "" => Console::write_line(STR_SLICE_8),
        x if x.len() > 3 => Console::write_line(FeString::from_owned(format!("{}: 6", x))),
        _ => Console::write_line(STR_SLICE_9),
    }

    let x = true;
    Console::write_line(if x { STR_SLICE_10 } else { STR_SLICE_11 });

    let x: Option<bool> = Some(true);
    Console::write_line(
        x.map(|x| FeString::from_owned(x.to_string()))
            .unwrap_or(STR_SLICE_12),
    );

    Console::write_line(if let Some(x) = x {
        if x {
            STR_SLICE_13
        } else {
            STR_SLICE_14
        }
    } else {
        STR_SLICE_12
    });
}
