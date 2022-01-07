mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{ Console };

struct Person {
  name: Shareable<LangString>,
  age: Shareable<usize>,
}
impl Person {
  pub fn new(name: Shareable<LangString>, age: Shareable<usize>) -> Self {
    Self { name, age }
  }
}
impl std::ops::Drop for Person {
  fn drop(&mut self) {
    Console::write_line(LangString::from_owned(format!("Dropping {}", self.name)));
  }
}

fn get_oldest_person<'ab>(person1: &'ab Person, person2: &'ab Person) -> Option<&'ab Person> {
  if person1.age > person2.age {
    return Some(person1);
  }

  if person1.age < person2.age {
    return Some(person2);
  }

  return None;
}

fn test_lifetimes<'ac, 'b>(
  person1: Person,
  person2: &'ac Person,
  person3: &'b Person,
  person4: Person,
  person5: &'ac Person,
) -> &'ac Person {
  return get_oldest_person(&person2, &person5).unwrap();
}

fn main() {
  {
    let person1 = Person { name: Shareable::of_unique(LangString::from_slice("Madison")), age: Shareable::of_unique(23), };
    let person2 = Person { name: Shareable::of_unique(LangString::from_slice("Adam")), age: Shareable::of_unique(25), };

    let oldest = get_oldest_person(&person1, &person2).unwrap();

    Console::write_line(LangString::from_owned(format!("Oldest person is {} at age {}", oldest.name, oldest.age)));
  }

  let person4 = Person { name: Shareable::of_unique(LangString::from_slice("four")), age: Shareable::of_unique(4) };
  let person3 = Person { name: Shareable::of_unique(LangString::from_slice("three")), age: Shareable::of_unique(3) };

  let res;

  {
    let person2 = Person { name: Shareable::of_unique(LangString::from_slice("two")), age: Shareable::of_unique(2) };
    let person5 = Person { name: Shareable::of_unique(LangString::from_slice("five")), age: Shareable::of_unique(5) };

    {
      let person1 = Person { name: Shareable::of_unique(LangString::from_slice("one")), age: Shareable::of_unique(1) };

      res = test_lifetimes(
        person1,
        &person2,
        &person3,
        person4,
        &person5,
      );
    }

    Console::write_line(LangString::from_owned(format!("Res: {}", res.name)));
  }

  Console::write_line(LangString::from_slice("After drops"));

  let name = Shareable::of_shared(LangString::from_slice("Adam"));

  let person1 = Person { name: name.share(), age: Shareable::of_unique(24) };
  let person2 = Person { name: name.share(), age: Shareable::of_unique(25) };
}
