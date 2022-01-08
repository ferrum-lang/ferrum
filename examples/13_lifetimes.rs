mod lang_prelude;
mod lang_std;

use lang_prelude::*;
use lang_std::{ Console };

#[allow(non_upper_case_globals)]
const STR_SLICE_0: LangString = LangString::from_slice("Madison");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: LangString = LangString::from_slice("Adam");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: LangString = LangString::from_slice("four");

#[allow(non_upper_case_globals)]
const STR_SLICE_3: LangString = LangString::from_slice("three");

#[allow(non_upper_case_globals)]
const STR_SLICE_4: LangString = LangString::from_slice("two");

#[allow(non_upper_case_globals)]
const STR_SLICE_5: LangString = LangString::from_slice("five");

#[allow(non_upper_case_globals)]
const STR_SLICE_6: LangString = LangString::from_slice("one");

#[allow(non_upper_case_globals)]
const STR_SLICE_7: LangString = LangString::from_slice("After drops");

struct Person {
  pub name: LangString,
  pub age: usize,
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
    let person1 = Person { name: STR_SLICE_0, age: 23, };
    let person2 = Person { name: STR_SLICE_1, age: 25, };

    let oldest = get_oldest_person(&person1, &person2).unwrap();

    Console::write_line(LangString::from_owned(format!("Oldest person is {} at age {}", oldest.name, oldest.age)));
  }

  let person4 = Person { name: STR_SLICE_2, age: 4 };
  let person3 = Person { name: STR_SLICE_3, age: 3 };

  let res;

  {
    let person2 = Person { name: STR_SLICE_4, age: 2 };
    let person5 = Person { name: STR_SLICE_5, age: 5 };

    {
      let person1 = Person { name: STR_SLICE_6, age: 1 };

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

  Console::write_line(STR_SLICE_7);
}
