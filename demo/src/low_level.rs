use crate::ferrum::prelude::*;

pub fn do_something_crazy(name: FeStr, age: FeStr) -> FeStr {
    let name = name.to_string();
    let age = age.to_string();
    let age = age.parse::<usize>().unwrap();

    let person = Person {
        name,
        age,
    };

    return FeStr::from_owned(format!("{person:#?}"));
}

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}