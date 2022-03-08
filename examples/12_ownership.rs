#![feature(const_fn_trait_bound)]

mod fe_prelude;
mod fe_std;

use fe_prelude::*;
use fe_std::Console;

#[allow(non_upper_case_globals)]
const STR_SLICE_0: FeShareable<FeString> = FeShareable::new(FeString::from_slice("Adam"));

#[allow(non_upper_case_globals)]
const STR_SLICE_1: FeShareable<FeString> = FeShareable::new(FeString::from_slice("Madeline"));

#[allow(non_upper_case_globals)]
const STR_SLICE_2: FeShareable<FeString> = FeShareable::new(FeString::from_slice("Brian"));

#[allow(non_upper_case_globals)]
const STR_SLICE_3: FeShareable<FeString> = FeShareable::new(FeString::from_slice(
    "Got unique person1 while person2 still exists!!!",
));

#[allow(non_upper_case_globals)]
const STR_SLICE_4: FeShareable<FeString> = FeShareable::new(FeString::from_slice("new"));

#[allow(non_upper_case_globals)]
const STR_SLICE_5: FeShareable<FeString> =
    FeShareable::new(FeString::from_slice("person1 cannot be made unique yet."));

#[allow(non_upper_case_globals)]
const STR_SLICE_6: FeShareable<FeString> = FeShareable::new(FeString::from_slice(
    "Got unique person1 after person2 dropped.",
));

#[allow(non_upper_case_globals)]
const STR_SLICE_7: FeShareable<FeString> = FeShareable::new(FeString::from_slice(
    "person1 cannot be made unique even after person2 dropped?!",
));

#[derive(Debug, Clone)]
struct Person {
    pub name: FeShareable<FeString>,
    pub age: usize,
}
impl Person {
    fn new(name: FeShareable<FeString>, age: usize) -> Self {
        Self { name, age }
    }

    fn requires_borrow(&self) {}
    fn requires_borrow_mut(&mut self) -> &mut Self {
        return self;
    }
}

#[derive(Debug, Clone)]
struct House {
    pub owner: FeMutField<Person>,
}

#[derive(Debug, Clone)]
struct Child {
    parents: (FeShareable<Person>, FeShareable<Person>),
}

fn main() {
    let fe_person1 = Person {
        name: STR_SLICE_0,
        age: 25,
    };

    let fe_person2 = fe_person1;
    let mut shared_person2;

    let shared = FeShareable::new(fe_person2).share();
    shared_person2 = shared.0;
    let shared_person3 = shared.1;

    // `Shareables` must be `borrow`ed, or `borrow_mut`ed to access inner data
    Console::write_line(FeString::from_owned(format!(
        "{} is {} years old.",
        shared_person3.name, shared_person2.age
    )));

    let mut fe_person1 = Person {
        name: STR_SLICE_0,
        age: 25,
    };
    let mut shared_person1;

    let shared = FeShareable::new(fe_person1).share();
    shared_person1 = shared.0;
    let shared_person2 = shared.1;

    let shared_person3 = shared_person2;

    let mut fe_person1 = Person {
        name: STR_SLICE_0,
        age: 25,
    };

    let mut fe_person1 = shared_person1.clone().take();

    fe_person1.age = 26;

    Console::write_line(FeString::from_owned(format!(
        "{} is {} years old.",
        shared_person3.name, fe_person1.age
    )));

    fe_person1.requires_borrow();
    fe_person1.requires_borrow_mut();

    let mut house1 = House {
        owner: FeMutField::new(fe_person1),
    };
    let house2 = House {
        owner: FeMutField::from(shared_person3),
    };

    house1.owner.age = 50;

    let shared_mom = FeShareable::new(Person {
        name: STR_SLICE_1,
        age: 52,
    });
    let shared_dad = FeShareable::new(Person {
        name: STR_SLICE_2,
        age: 56,
    });

    let shared_adam = FeShareable::new(Child {
        parents: (shared_mom, shared_dad),
    });

    let mut shared_person1 = FeShareable::new(Person {
        name: STR_SLICE_0,
        age: 25,
    });

    {
        let shared = shared_person1.share();
        shared_person1 = shared.0;
        let shared_person2 = shared.1;

        match shared_person1.try_mutable() {
            Ok(person) => {
                Console::write_line(STR_SLICE_3);
                shared_person1 = FeShareable::new(Person {
                    name: STR_SLICE_4,
                    age: 0,
                });
            }
            Err(person) => {
                Console::write_line(STR_SLICE_5);
                shared_person1 = person;
            }
        }
    }

    match shared_person1.try_mutable() {
        Ok(person) => {
            Console::write_line(STR_SLICE_6);
        }
        Err(person) => {
            Console::write_line(STR_SLICE_7);
        }
    }
}
