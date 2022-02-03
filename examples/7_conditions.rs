mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::Console;

#[allow(non_upper_case_globals)]
const STR_SLICE_0: LangString = LangString::from_slice("true");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: LangString = LangString::from_slice("false");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: LangString = LangString::from_slice("else");

#[allow(non_upper_case_globals)]
const STR_SLICE_3: LangString = LangString::from_slice("Adam");

#[allow(non_upper_case_globals)]
const STR_SLICE_4: LangString = LangString::from_slice("1");

#[allow(non_upper_case_globals)]
const STR_SLICE_5: LangString = LangString::from_slice("2");

#[allow(non_upper_case_globals)]
const STR_SLICE_6: LangString = LangString::from_slice("3");

#[allow(non_upper_case_globals)]
const STR_SLICE_7: LangString = LangString::from_slice("4");

#[allow(non_upper_case_globals)]
const STR_SLICE_8: LangString = LangString::from_slice("5");

#[allow(non_upper_case_globals)]
const STR_SLICE_9: LangString = LangString::from_slice("7");

#[allow(non_upper_case_globals)]
const STR_SLICE_10: LangString = LangString::from_slice("yes");

#[allow(non_upper_case_globals)]
const STR_SLICE_11: LangString = LangString::from_slice("no");

#[allow(non_upper_case_globals)]
const STR_SLICE_12: LangString = LangString::from_slice("none");

#[allow(non_upper_case_globals)]
const STR_SLICE_13: LangString = LangString::from_slice("true");

#[allow(non_upper_case_globals)]
const STR_SLICE_14: LangString = LangString::from_slice("false");

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
        Console::write_line(LangString::from_owned(y.to_string()));
    }

    match x {
        None => Console::write_line(STR_SLICE_5),
        _ => Console::write_line(STR_SLICE_6),
    }

    match "Adam" {
        "Bates" => Console::write_line(STR_SLICE_7),
        "" => Console::write_line(STR_SLICE_8),
        x if x.len() > 3 => Console::write_line(LangString::from_owned(format!("{}: 6", x))),
        _ => Console::write_line(STR_SLICE_9),
    }

    let x = true;
    Console::write_line(if x { STR_SLICE_10 } else { STR_SLICE_11 });

    let x: Option<bool> = Some(true);
    Console::write_line(
        x.map(|x| LangString::from_owned(x.to_string()))
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
