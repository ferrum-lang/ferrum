mod fe_prelude;
mod fe_std;

use fe_prelude::*;

#[allow(non_upper_case_globals)]
const STR_SLICE_0: FeString = FeString::from_slice("Adam");

#[allow(non_upper_case_globals)]
const STR_SLICE_1: FeString = FeString::from_slice("Adam Bates");

#[allow(non_upper_case_globals)]
const STR_SLICE_2: FeString = FeString::from_slice("Person");

#[allow(non_upper_case_globals)]
const STR_SLICE_3: FeString = FeString::from_slice("Bates");

#[allow(non_upper_case_globals)]
const STR_SLICE_4: FeString = FeString::from_slice("ADAM");

#[allow(non_upper_case_globals)]
const STR_SLICE_5: FeString = FeString::from_slice("Other");

#[allow(non_upper_case_globals)]
const STR_SLICE_6: FeString = FeString::from_slice("BATES");

struct Person {
    pub name: FeString,
}

fn test_person() {
    let person = Person { name: STR_SLICE_0 };

    // --- //

    let person = Person { name: STR_SLICE_0 };

    let person = Person { name: STR_SLICE_1 };

    // --- //

    let mut person = Person { name: STR_SLICE_0 };

    person.name = STR_SLICE_1;

    // --- //

    let mut person = Person { name: STR_SLICE_0 };

    person.name = STR_SLICE_1;

    let mut person = Person { name: STR_SLICE_1 };
}

struct PartiallyMutableExample {
    pub first: FeString,
    pub second: FeString,
    pub third: Person,
}

fn test_partial_mutable_example() {
    let mut person = Person { name: STR_SLICE_2 };

    let mut example = PartiallyMutableExample {
        first: STR_SLICE_0,
        second: STR_SLICE_3,
        third: person,
    };

    example.first = STR_SLICE_4;

    example.third.name = STR_SLICE_5;

    let person = Person { name: STR_SLICE_2 };

    let example = PartiallyMutableExample {
        first: STR_SLICE_0,
        second: STR_SLICE_3,
        third: person,
    };
}

struct ForcedPartiallyMutableExample {
    pub first: FeString,
    pub second: FeString,
}

fn test_forced_partially_mutable_example() {
    let mut example = ForcedPartiallyMutableExample {
        first: STR_SLICE_0,
        second: STR_SLICE_3,
    };

    example.first = STR_SLICE_4;
}

struct MutableExample {
    pub first: FeString,
    pub second: FeString,
}

fn test_mutable_example() {
    let mut example = MutableExample {
        first: STR_SLICE_0,
        second: STR_SLICE_3,
    };

    example.first = STR_SLICE_4;
    example.second = STR_SLICE_6;

    let example = MutableExample {
        first: STR_SLICE_0,
        second: STR_SLICE_3,
    };
}

struct ForcedMutableExample {
    pub first: FeString,
    pub second: FeString,
}

fn test_forced_mutable_example() {
    let mut example = ForcedMutableExample {
        first: STR_SLICE_0,
        second: STR_SLICE_3,
    };

    example.first = STR_SLICE_4;
    example.second = STR_SLICE_6;
}

struct ImmutableExample {
    pub first: FeString,
    pub second: FeString,
}

fn test_immutable_example() {
    let example = ImmutableExample {
        first: STR_SLICE_0,
        second: STR_SLICE_3,
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
