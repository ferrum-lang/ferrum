mod fe_prelude;
mod fe_std;

use fe_prelude::*;
use fe_std::Console;

fn main() {
    let mut i = 0;
    loop {
        if i >= 10 {
            break;
        }

        Console::write_line(FeString::from_owned(format!("loop: {}", i)));

        i += 1;
    }

    let mut i = 0;
    while i < 10 {
        Console::write_line(FeString::from_owned(format!("while: {}", i)));
        i += 1;
    }

    for i in 0..10 {
        Console::write_line(FeString::from_owned(format!("for excl: {}", i)));
    }

    for i in 0..=10 {
        Console::write_line(FeString::from_owned(format!("for incl: {}", i)));
    }

    let x = vec![1, 2, 3];

    for n in x.iter() {
        Console::write_line(FeString::from_owned(format!("for in arr: {}", n)));
    }

    for (idx, n) in x.iter().enumerate() {
        Console::write_line(FeString::from_owned(format!(
            "for with idx: ({}, {})",
            idx, n
        )));
    }
}
