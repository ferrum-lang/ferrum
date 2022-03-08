#![feature(const_fn_trait_bound)]

mod fe_prelude;
mod fe_std;

use fe_prelude::*;
use fe_std::Console;

fn main() {
    let x = FeShareable::new(vec![1, 2, 3]);
    let x = FeShareable::new(vec![1, 2, 3]);

    let x = {
        let mut tmp = vec![1, 1];

        let mut x = x.clone().take();
        tmp.append(&mut x);

        tmp.push(4);

        FeShareable::new(tmp)
    };

    let x = FeShareable::new(vec![
        FeShareable::new(vec![0]),
        FeShareable::new(vec![1, 2, 3]),
        FeShareable::new(vec![4, 5, 6]),
        FeShareable::new(vec![7, 8, 9]),
        FeShareable::new(vec![10]),
    ]);

    let x = FeShareable::new(vec![1, 2]);

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
