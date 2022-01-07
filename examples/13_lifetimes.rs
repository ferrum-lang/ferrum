mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{ Console };

struct Person {
  name: LangString,
  age: usize,
}
impl Person {
  pub fn new(name: LangString, age: usize) -> Self {
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
    let person1 = Person { name: LangString::from_slice("Madison"), age: 23, };
    let person2 = Person { name: LangString::from_slice("Adam"), age: 25, };

    let oldest = get_oldest_person(&person1, &person2).unwrap();

    Console::write_line(LangString::from_owned(format!("Oldest person is {} at age {}", oldest.name, oldest.age)));
  }

  let person4 = Person { name: LangString::from_slice("four"), age: 4 };
  let person3 = Person { name: LangString::from_slice("three"), age: 3 };

  let res;

  {
    let person2 = Person { name: LangString::from_slice("two"), age: 2 };
    let person5 = Person { name: LangString::from_slice("five"), age: 5 };

    {
      let person1 = Person { name: LangString::from_slice("one"), age: 1 };

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
}
