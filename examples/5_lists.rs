mod fe_prelude;
mod fe_std;

use fe_prelude::*;
use fe_std::Console;

fn main() {
    let x = vec![1, 2, 3];
    let x = vec![1, 2, 3];

    let x = {
        let mut tmp = vec![1, 1];

        let mut x = x;
        tmp.append(&mut x);

        tmp.push(4);

        tmp
    };

    let x = vec![
        vec![0],
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10],
    ];

    let x = [1, 2];

    Console::write_line(FeString::from_owned(format!("{}, {}", x[0], x[1])));

    Console::write_line(FeString::from_owned(
        vec![1, 2, 3]
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(", "),
    ));

    Console::write_line(FeString::from_owned(
        vec![1, 2, 3]
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("_"),
    ));
}
