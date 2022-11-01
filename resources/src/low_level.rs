use super::other;

pub fn low_level_func(name: impl Into<String>, age: impl Into<String>) {
    println!("low_level_func");

    let name: String = name.into();
    
    let age: String = age.into();
    let age: usize = age.parse().unwrap();

    let person = Person { name, age };

    other::foo();

    println!("{person:#?}");
}

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

