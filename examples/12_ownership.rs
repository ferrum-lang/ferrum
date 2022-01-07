mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{ Console };

struct Person {
  name: LangString,
  age: usize,
}
impl Person {
  fn requires_borrow(&self) {}
  fn requires_borrow_mut(&mut self) {}
  fn requires_own(self) -> Self { self }
  fn requires_own_mut(mut self) -> Self { self }
}

struct House {
  owner: Shareable<Person>,
}

struct Child {
  parents: (Person, Person),
}

fn main() {
  // `person1` not `Shareable` because source code never uses `share` and never passes as a `Shareable`
  // But if `person1` was shared or passed as a `Shareable`, the transpiled code would wrap in `Shareable::new`
  let person1 = Person {
    name: LangString::from_slice("Adam"),
    age: 25,
  }.requires_own_mut();

  let person1 = person1.requires_own();

  let person2 = Shareable::new(person1);

  let person3 = person2.share();

  // `Shareables` must be `borrow`ed, or `borrow_mut`ed to access inner data
  Console::write_line(LangString::from_owned(format!("{} is {} years old.", person3.borrow().name, person2.borrow().age)));

  let mut person1 = Shareable::new(Person {
    name: LangString::from_slice("Adam"),
    age: 25,
  });

  let person2 = person1.share();

  let person3 = person2;

  person1.borrow_mut().age = 26;

  Console::write_line(LangString::from_owned(format!("{} is {} years old.", person1.borrow().name, person3.borrow().age)));

  person1.borrow().requires_borrow();
  person1.borrow_mut().requires_borrow_mut();

  let mut house1 = House { owner: person1.share() };
  let house2 = House { owner: person1.share() };

  let house4 = House { owner: person3.share() };

  house1.owner.borrow_mut().age = 50;

  // No need to `Shareable<Person>` here because `unique`
  let mom = Person { name: LangString::from_slice("Madeline"), age: 52, };
  let dad = Person { name: LangString::from_slice("Brian"), age: 56, };

  let adam = Child {
    parents: (mom, dad),
  };

  let mut person1 = Shareable::new(Person {
    name: LangString::from_slice("Adam"),
    age: 25,
  });

  {
    let person2 = person1.share();

    let opt_unique_person = person1.try_unique();

    match opt_unique_person {
      Ok(person) => {
        Console::write_line(LangString::from_slice("Got unique person1 while person2 still exists!!!"));
        person.requires_own();
        person1 = Shareable::new(Person { name: LangString::from_slice("new"), age: 0, });
      },
      Err(person) => {
        Console::write_line(LangString::from_slice("person1 cannot be made unique yet."));
        person1 = person;
      }
    }
  }

  let opt_unique_person = person1.try_unique();

  match opt_unique_person {
    Ok(person) => {
      Console::write_line(LangString::from_slice("Got unique person1 after person2 dropped."));
      person.requires_own();
    },
    Err(person) => {
      Console::write_line(LangString::from_slice("person1 cannot be made unique even after person2 dropped?!"));
    }
  }
}
