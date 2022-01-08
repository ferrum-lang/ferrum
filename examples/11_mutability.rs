mod lang_prelude;
mod lang_std;

use lang_prelude::*;

struct Person {
  pub name: LangString,
}

fn test_person() {
  let person = Person { name: LangString::from_slice("Adam"), };

  // --- //

  let person = Person { name: LangString::from_slice("Adam"), };

  let person = Person { name: LangString::from_slice("Adam Bates"), };

  // --- //

  let mut person = Person { name: LangString::from_slice("Adam"), };

  person.name = LangString::from_slice("Adam Bates");

  // --- //

  let mut person = Person { name: LangString::from_slice("Adam"), };

  person.name = LangString::from_slice("Adam Bates");

  let mut person = Person { name: LangString::from_slice("Adam Bates") };
}

struct PartiallyMutableExample {
  pub first: LangString,
  pub second: LangString,
  pub third: Person,
}

fn test_partial_mutable_example() {
  let mut person = Person { name: LangString::from_slice("Person"), };

  let mut example = PartiallyMutableExample {
    first: LangString::from_slice("Adam"),
    second: LangString::from_slice("Bates"),
    third: person,
  };

  example.first = LangString::from_slice("ADAM");

  example.third.name = LangString::from_slice("Other");

  let person = Person { name: LangString::from_slice("Person"), };

  let example = PartiallyMutableExample {
    first: LangString::from_slice("Adam"),
    second: LangString::from_slice("Bates"),
    third: person,
  };
}

struct ForcedPartiallyMutableExample {
  pub first: LangString,
  pub second: LangString,
}

fn test_forced_partially_mutable_example() {
  let mut example = ForcedPartiallyMutableExample {
    first: LangString::from_slice("Adam"),
    second: LangString::from_slice("Bates"),
  };

  example.first = LangString::from_slice("ADAM");
}

struct MutableExample {
  pub first: LangString,
  pub second: LangString,
}

fn test_mutable_example() {
  let mut example = MutableExample {
    first: LangString::from_slice("Adam"),
    second: LangString::from_slice("Bates"),
  };

  example.first = LangString::from_slice("ADAM");
  example.second = LangString::from_slice("BATES");

  let example = MutableExample { first: LangString::from_slice("Adam"), second: LangString::from_slice("Bates") };
}

struct ForcedMutableExample {
  pub first: LangString,
  pub second: LangString,
}

fn test_forced_mutable_example() {
  let mut example = ForcedMutableExample {
    first: LangString::from_slice("Adam"),
    second: LangString::from_slice("Bates"),
  };

  example.first = LangString::from_slice("ADAM");
  example.second = LangString::from_slice("BATES");
}

struct ImmutableExample {
  pub first: LangString,
  pub second: LangString,
}

fn test_immutable_example() {
  let example = ImmutableExample {
    first: LangString::from_slice("Adam"),
    second: LangString::from_slice("Bates"),
  };
}

fn main() {
  test_person();
  test_partial_mutable_example();
  test_forced_partially_mutable_example();
  test_mutable_example();
  test_forced_mutable_example();
  test_immutable_example();
}
