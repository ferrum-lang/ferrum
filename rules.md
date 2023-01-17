# Some Language Rules
An incomplete collection of the language's rules and design decisions to help me keep track.

## `let` - Mutable Variable Declarations

Whenever `let` is used to declare a variable, it equates to `let mut` in Rust.

Note: Any normal declarations that have any mutability (shallow or deep) must use `let`

Ferrum:
```rust
let x = "abc"

let y = &x
y = &"cba"

let z

let y = &mut x
@y = "cba"
y.append('!')
```

Rust: 
```rust
let mut x = FeString::from("abc");

let mut y = &x;
y = &FeString::from("cba");

let mut z;

let mut y = &mut x;
*y = FeString::from("cba");
y.append('!');
```

## `const` - Immutable Variable Declarations

Whenever `const` is used to declare a variable, it equates to `let` in Rust.

Note: `const` can only be used on non-mutable data, whether owned or shared reference

Ferrum:
```rust
const x = "abc"
const y = &x
const z

// The below line is invalid due to mutability:
// const y = &mut x
```

Rust:
```rust
let x = FeString::from("abc");
let y = &x;
let z;
```

## `std::mem` - Smart Pointers

The `std::mem` package provides smart pointers for managing data in more complex manors.

Ferrum:
```rust
use std::mem::HeapRef

// Heap-allocation (ie. Boxing in Rust)
// Useful if large data is moved around on the stack a lot,
//   better to move around stack pointer to large data on heap.
const name: HeapRef<String> = HeapRef("Adam")
```

```rust
use std::mem::SharedMut

// Allows shared mutable access, without extra memory overhead
const name: SharedMut<String> = SharedMut("Adam")

let name_ref: &mut String = SharedMut::get_mut(&name)
let name_ref_2: &mut String = SharedMut::get_mut(&name)
```

```rust
use std::mem::Rc

// Eager immutable reference counting
const name: Rc<String> = Rc("Adam")
const name2: Rc<String> = Rc::share(&name)

// Lazy immutable reference counting
const name = Rc("Adam", lazy = true) // Still on stack, not reference counting yet
const name2 = Rc::share(&name)       // Reference counting starts here
```

```rust
use std::mem::{Rc, Mutex}

// Reference counting, with runtime check for mutability
const name: Rc<Mutex<String>> = Rc(Mutex("Adam"))
const name2: Rc<Mutex<String>> = Rc::share(&name)

let name_lock: MutexLock<String> = name.await_lock(timeout_ms = 50)!
let name_ref: &mut String = name_lock.open() // &mut String

let name_lock2: ?MutexLock<String> = name2.try_lock() ?? print("cant get mutable name2")
let name_ref2: ?&mut String = name_lock2?.get() // None

print("{name_ref}, {name_ref2:?}")
```

```rust
use std::mem::Rc

// Mutable reference counting, allows shared mutable access
const name: Rc<SharedMut<String>> = Rc("Adam")   // lazy option available here too
const name2: Rc<SharedMut<String>> = Rc::share(&mut name)
```

```rust
use std::mem::Gc

// Garbage collection
const name: Gc<String> = Gc("Adam") // lazy option available here too
const name2: Gc<String> = Gc::share(&name)

Gc::force_collect() // can be called at any point to clean up unused memory

// Note: GC normally cleans up automatically in background threads. Manual clean up isn't necessary.
// Best off doing something like this to manually clean up:

const gc_task = Async::start_soon(Gc::force_collect) // Start background clean up

// .. some code here ..

gc_task.await() // Wait for background clean up to be finished
```

```rust
use std::mem::{Gc, Mutex}

// Garbage collection, with runtime check for mutability
const name: Gc<Mutex<String>> = Gc(Mutex("Adam")) // lazy option available here too
const name2: Gc<Mutex<String>> = Gc::share(&name)

let name_lock: MutexLock<String> = name.await_lock(timeout_ms = 50)!
let name_ref: &mut String = name_lock.open() // &mut String

let name_lock2: ?MutexLock<String> = name2.try_lock() ?? print("cant get mutable name2")
let name_ref2: ?&mut String = name2?.open() // None

print("{name_ref}, {name_ref2:?}")

Gc::force_collect() // can be called at any point to clean up unused memory
```

```rust
use std::mem::{Gc, SharedMut}

// Mutable garbage collection, allows shared mutable access
const name: Gc<SharedMut<String>> = Gc(SharedMut("Adam")) // lazy option available here too
const name2: Gc<SharedMut<String>> = Gc::share(&name)

Gc::force_collect() // can be called at any point to clean up unused memory
```

## `std::tasks` - Smart Pointers

The `std::tasks` package provides structs and functions to work ith asynchronous tasks.

Ferrum:

```rust
use std::tasks::Async

const task = Async::start_soon(() => print(""))

task.await()
```

```rust
use std::{
    tasks::{AsyncTasks, sleep},
    time::Duration,
}

let tasks = AsyncTasks()

tasks.start_soon(() => do
    sleep(Duration::from_ms(100))

    print("Thread 1")
;)

tasks.start_soon(() => print("Thread 2"))

tasks.await_all()
```


## `std::prelude::*` -  Good default imports

```rust
use std::prelude::*

// equivalent to:
use std::{
    self,
    tasks::{Async, AsyncTasks, sleep},
    mem::{Rc, Gc, SharedMut, Mutex},
    String,
    Map,
    Set,
    clone,
    // etc ...
}
```


## `fn` - Functions

Functions are defined using `fn`.

Notes:
- The main function is optional. Top-level statements are only supported in an entry file with no main function.
- Owned parameters are mutable by default
- Reference parameters have shallow immutability. Deep mutability depends on reference type (`&` vs `&mut`).

Ferrum:
```rust
fn main()
    print("hello world")

    foo_bar()

    let name = get_name()

    // named params in any order
    say_hello(age = 25, &name)

    // optional and default params can be skipped
    const updated = update_name(&mut name)

    if updated
        consume_name(name)
    ;
;

fn foo_bar()
    fn inner()
        print("Hello from inner")
    ;

    print("Hello from foo_bar")

    inner()
;

fn get_name() -> String
    return "Adam"
;

fn say_hello(name: &String, age: uint)
    print("Hello, {age} year old named {name}")
;

fn update_name(name: &mut String, force: bool = false) -> bool
    if !force && @name != "Adam"
        return false
    ;

    name.append(" Bates")

    return true
;

fn consume_name(name: String)
    name.append('!')

    print("consumed: {name}")
;
```

Rust:
```rust
fn main() {
    print("hello world");

    foo_bar();

    let mut name = get_name();

    say_hello(&name, FeUint::from(25));

    let updated = update_name(&mut name, None);

    if updated {
        consume_name(name);
    }
}

fn foo_bar() {
    fn inner() {
        print("Hello from inner");
    }

    print("Hello from foo_bar");
}

fn get_name() -> FeString {
    return FeString::from("Adam");
}

fn say_hello(name: &FeString, age: FeUint) {
    print(format!("Hello, {} year old named {}", age, name));
}

fn update_name(name: &mut FeString, force: Option<bool>) -> bool {
    let force = force.unwrap_or_else(|| false);

    if !force && *name != FeString::from("Adam") {
        return false;
    }

    name.append(FeString::from(" Bates"));

    return true;
}

fn consume_name(mut name: FeString) {
    name.append('!');

    print(format!("consumed: {}", name));
}
```

## `struct` - Structs

Create concrete data structures to hold data, and/or implement methods.

Ferrum:
```rust
type Serial = uint

struct Device {
    serial: Serial,
    is_active: bool = true,
}

struct Inventory {
    devices: Map<Serial, Device> = Map(),
}

impl
    &self.get_all() -> [&Device]
        return self.devices.values()
    ;

    &mut self.add(device: Device)
        self.devices.insert(device.serial, device);
    ;

    self.decorate() -> Self
        return Inventory(self.devices)
    ;

    mut self.with_device(device: Device) -> Self
        self.add(device)

        return self
    ;
;

const inventory = Inventory()
```

Rust:
```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Serial(FeUint);

impl<T: Into<FeUint>> From<T> for Serial {
    fn from(value: T) -> Self {
        return Self(value.into());
    }
}

impl Copy for Serial {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Device {
    serial: Serial,
    is_active: true,
}

impl Device {
    fn new(serial: Serial, is_active: Option<bool>) -> Self {
        return Self {
            serial,
            is_active: is_active.unwrap_or_else(|| true),
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Inventory {
    devices: FeMap<Serial, Device>,
}

impl Inventory {
    fn new(devices: Option<FeMap<Serial, Device>>) -> Self {
        return Self {
            devices: devices.unwrap_or_else(|| FeMap::new()),
        };
    }

    fn get_all(&self) -> FeList<&Device> {
        return self.devices.values();
    }

    fn add(&mut self, device: Device) {
        self.devices.insert(device.serial, device);
    }

    fn decorate(self) -> Self {
        return Inventory {
            devices: self.devices,
        };
    }

    fn with_device(mut self, device: Device) -> Self {
        self.add(device);

        return self;
    }
}

fn main() {
    let inventory = Inventory::new(None);
}
```

## `trait` - Traits

Create abstract traits that can be implemented by structs.

Note: Trait implementations must be directly after the trait definition, or the struct definition

Ferrum:
```rust
trait Serializable
    &self.serialize() -> String

    // Default implementations
    // Also 'static' methods
    impl fn serialize_all(values: ~Iter<Self>) -> [String]
        return values.map(Self::serialize)
    ;
;
// impl for <struct> when after the trait def
impl for User
    &self.serialize() -> _
        => "User \{ name: {self.name}, age: {self.age} }"
;

struct User {
    name: String,
    age: uint,
}

struct Device {
    serial_number: String,
}
impl
    &self.get_serial_num() -> &String
        return &self.serial_number
    ;

    &mut self.set_serial_num(serial_number: String)
        self.serial_number = serial_number
    ;
;
// impl <trait> when after the struct def
impl Serializable
    &self.serialize() -> String
        return "Device \{ serial_number: {self.serial_number} }"
    ;
;
```

Rust:
```rust
trait Serializable {
    fn serialize(&self) -> FeString;

    fn serialize_all(values: dyn FeIter<Self>) -> FeVec<String> {
        return values.map(Self::serialize).collect();
    }
}
impl Serializable for User {
    fn serialize(&self) -> FeString {
        return FeString::from_owned(format!("User \{ name: {}, age: {} }", self.name, self.age));
    }
}

struct User {
    name: FeString,
    age: FeUint,
}

struct Device {
    serial_number: FeString,
}
impl Device {
    fn get_serial_num(&self) -> &FeString {
        return &self.serial_number;
    }

    fn set_serial_num(&mut self, serial_number: FeString) {
        self.serial_number = serial_number;
    }
}
impl Serializable for Device {
    fn serialize(&self) -> FeString {
        return FeString::from_owned(format!("Device \{ serial_number: {} }", self.serial_number));
    }
}
```

## `enum` - Enums

Create enums.

Ferrum:
```rust
enum MyEnum {
    MyEmptyVal,
    MyTupleVal(int, char, char),
    MyStructVal {
        name: String,
        age: uint,
    } = "some value",
}

const e: MyEnum = MyEnum::MyTupleVal(1, 'a', 'b')

if e.value() is Some(value)
    print(value)
;
```

Rust:
```rust

const VAL_2: FeString = FeStr::from_slice("some_value");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MyEnum {
    MyEmptyVal,
    MyTupleVal(FeInt, char, char),
    MyStructVal { name: FeString, name: FeUint },
}

impl MyEnum {
    pub fn value(&self) -> Option<&'static FeString> {
        return match *self {
            Self::MyEmptyVal => None,
            Self::MyTupleVal(..) => None,
            Self::MyStructVal { .. } => Some(&VAL_2),
        };
    }
}

fn main() {
    let e: MyEnum = MyEnum::MyTupleVal(FeInt::from(1), 'a', 'b');

    if let Some(value) = e.value() {
        print(value);
    }
}
```

## `alias` - Type-Aliasing

`alias` syntax creates type aliases to make referencing long or complicated types easier.

Ferrum:
```rust
alias Cache = Map<(char, char, int), ~Iter<String>>

fn get_some() -> ?Map<(char, char, int), ~Iter<String>>
    return None
;

fn accept_some(maybe_cache: ?Cache)
    # TODO
;

const cache = get_some()

// Cache is just an alias
accept_some(cache)
```

Rust:
```rust
type Cache = FeMap<(char, char, FeInt), Box<dyn FeIter<FeShared<FeString>>>>;

fn get_some() -> Option<FeMap<(char, char, FeInt), Box<dyn FeIter<FeShared<FeString>>>>> {
    return None;
}

fn accept_some(maybe_cache: Option<Cache>) {
    todo!();
}

fn main() {
    let cache = get_some();

    accept_some(cache);
}
```

## `type` - Type-Wrapping

`type` syntax creates an new unique type that wraps some other type. This can be useful for the newtype pattern.

Ferrum:
```rust
type Name = String
type Email = String
type Age = uint

struct Person {
    name: Name,
    email: Email,
    age: Age,
}

// coerced creation
const name: Name = "Adam"

// explicit creation
const email = Email("adam@example.com")

// inferred type by usage below
const age = 101

let person = Person(name, email, age)


const full_name: Name = "Adam Bates"

// won't compile as Name and Email are unique types, even though they both wrap String
// person.email = full_name
```

Rust:
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Name(FeString);

impl<T: Into<FeString>> From<T> for Name {
    fn from(value: T) -> Self {
        return Self(value.into());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Email(FeString);

impl<T: Into<FeString>> From<T> for FeStr {
    fn from(value: T) -> Self {
        return Self(value.into());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Age(FeUint);

impl Copy for Age {}

impl<T: Into<FeUint>> From<T> for FeString {
    fn from(value: T) -> Self {
        return Self(value.into());
    }
}

struct Person {
    name: Name,
    email: Email,
    age: Age,
}

impl Person {
    pub fn new(name: Name, email: Email, age: Age) -> Self {
        return Self {
            name,
            email,
            age,
        };
    }
}

fn main() {
    let name: Name = Name::from(FeString::from("Adam"));

    let email = Email(FeString::from("adam@example.com"));

    let age = Age(FeUint::from(101));

    let mut person = Person::new(name, email, age);

    let full_name: Name = Name::from(FeString::from("Adam Bates"));
}
```

## `&` - Shared References

Represent shared (immutable) references using the `&` syntax.

Ferrum:
```rust
const name = "Adam"

const borrow_name_1 = &name
const borrow_name_2 = &name

print(borrow_name_1, borrow_name_2)

fn get_largest_first_2(
    value1: -> &String, // ->& syntax tells us which references are used in return (helps w/ lifetimes)
    value2: -> &String,
    value3: &String,
) -> &String
    print(value3)

    if value1.len() >= value2.len()
        return value1
    else
        return value2
    ;
;

const value1 = "a"
const value2 = "bb"

const largest = do
    const value3 = "hello"

    yield get_largest_first_2(&value1, &value2, &value3)
;

// value3 has been dropped but this is still valid
print(largest)

fn inferred(value: &String) -> &String
    return value
;

struct HoldingRefs(&String, &[int]) impl
    &self.get_values() -> (&str, &[int])
        return (&self.0, &self.1)
    ;
;
```

Rust:
```rust
fn get_largest_first_2<'a>(
    value1: &'a FeString,
    value2: &'a FeString,
    value3: &FeString,
) -> &'a FeString {
    print(value3);

    if value1.len() >= value2.len() {
        return value1;
    } else {
        return value2;
    }
}

fn inferred(value: &FeString) -> &FeStr {
    return value;
}

struct HoldingRefs<'a, 'b>(&'a String, &'b Vec<isize>);

impl<'a> HoldingRefs<'a, 'a> {
    fn get_values(&'a self) -> (&'a String, &'a Vec<isize>) {
        return (&self.0, &self.1);
    }
}

fn main() {
    let name = FeString::from("Adam");

    let borrow_name_1 = &name;
    let borrow_name_2 = &name;

    print(format!("{}, {}", borrow_name_1, borrow_name_2));

    let value1 = FeString::from("a");
    let value2 = FeString::from("bb");

    let largest;
    {
        let value3 = FeString::from("hello");

        largest = get_largest_first_2(&value1, &value2, &value3);
    }

    print(largest);
}
```

## `&mut` - Mutable References

Represent mutable (unique) references using the `&mut` syntax.

Ferrum:
```rust
const name = "Adam"

// Won't compile: Cannot get mutable reference from a const
// let borrow_name_1 = &mut name

let name = name

let borrow_name_1 = &mut name

// Won't compile: cannot have multiple mutable references
// let borrow_name_2 = &mut name

print(borrow_name_1)

fn get_largest_first_2(
    value1: -> &mut String, // -> & syntax tells us which references are used in return (helps w/ lifetimes)
    value2: -> &mut String,
    value3: &mut String,
) -> &mut String
    print(value3)

    if value1.len() >= value2.len()
        return value1
    else
        return value2
    ;
;

let value1 = "a"
let value2 = "bb"

let largest = do
    let value3 = "hello"

    yield get_largest_first_2(&mut value1, &mut value2, &mut value3)
;

// value3 has been dropped but this is still valid
print(largest)

fn inferred(value: &String) -> &String
    return value
;

struct HoldingRefs(&mut String, &mut [int]) impl
    fn &mut self.get_values() -> (&mut String, &mut [int])
        return (&mut self.0, &mut self.1)
    ;
;
```

Rust:
```rust
fn get_largest_first_2<'a>(
    value1: &'a mut FeString,
    value2: &'a mut FeString,
    value3: &mut FeString,
) -> &'a mut FeString {
    print(value3);

    if value1.len() >= value2.len() {
        return value1;
    } else {
        return value2;
    }
}

fn inferred(value: &FeString) -> &FeStr {
    return value;
}

struct HoldingRefs<'a, 'b>(&'a mut String, &'b mut Vec<isize>);

impl<'a> HoldingRefs<'a, 'a> {
    fn get_values(&'a mut self) -> (&'a mut String, &'a mut Vec<isize>) {
        return (&mut self.0, &mut self.1);
    }
}

fn main() {
    let name = FeString::from("Adam");

    let mut name = name;

    let mut borrow_name_1 = &mut name;

    print(borrow_name_1);

    let mut value1 = FeString::from("a");
    let mut value2 = FeString::from("bb");

    let mut largest;
    {
        let mut value3 = FeString::from("hello");

        largest = get_largest_first_2(&mut value1, &mut value2, &mut value3);
    }

    print(largest);
}
```

## `?` - Optional

Represent optional values using the `?` syntax.

Ferrum:
```rust
const name: ?String = Some("Adam")
const name: ?String = None

// coerced
const name: ?String = "Adam"

fn get_length(value: ?&String) -> ?uint
    // ?. maps the optional
    const length: ?uint = value?.len()

    // ? short-circuts function, returning None if lhs is None
    const length: uint = length?

    // values can be coerced into Some(...)
    return length
;

const length = get_length(&name)
```

Rust:
```rust
fn get_length(value: Option<&FeString>) -> Option<usize> {
    let length: Option<usize> = value.map(|v| v.len());

    let length: usize = length?;

    return length.into();
}

fn main() {
    let name: Option<FeString> = Some(FeStr::from("Adam"));
    let name: Option<FeString> = None;

    let name: Option<FeString> = Some(FeStr::from("Adam"));
    
    let length = get_length(name.as_ref());
}
```

## `!` - Result

Represent results using `!` syntax.

Ferrum:
```rust
const name: !String = Ok("Adam")
const name: !String = Err(123)

const name: !String = "Adam"

fn get_length(value: !&String) -> !uint
    // !. maps the optional
    const length: !uint = value!.len()

    // ! short-circuts function, returning None if lhs is None
    const length: uint = length!

    // values can be coerced into ok(...)
    return length
;

const length = get_length(&name)
```

Rust:
```rust
fn get_length(value: FeResult<&FeString>) -> FeResult<usize> {
    let length: FeResult<usize> = value.map_ok(|v| v.len());

    let length: usize = length?;

    return length.into();
}

fn main() {
    let name: FeResult<FeString> = Ok(FeStr::from("Adam"));
    let name: FeResult<FeString> = Err(FeInt::from(123));

    let name: FeResult<FeString> = Ok(FeStr::from("Adam"));
    
    let length = get_length(name.as_ref());
}
```

## `[...]` - Lists

`[]` can be used to create lists.

Ferrum:
```rust
const values: [int] = [1, 2, 3]

// The above is syntactic-sugar for:
const values: List<int> = List(1, 2, 3)

// Iter is an interface that defines iteration over a collection
const values: ~Iter<int> = [1, 2, 3]

// Unfortunately, this requires moving the data onto the heap.
// This can be avoided by using references
const values: &~Iter<int> = &[1, 2, 3]

// Lists can be mutated
let values = []

for i in 0..10
    values.push(i)
;

print(values)
```

Rust:
```rust
let values: FeList<FeInt> = fe_list![FeInt::from(1), FeInt::from(2), FeInt::from(3)];

let values: FeList<FeInt> = fe_list![FeInt::from(1), FeInt::from(2), FeInt::from(3)];

let values: Box<dyn FeIter<FeInt>> = Box::new(fe_list![FeInt::from(1), FeInt::from(2), FeInt::from(3)]);

let values: &dyn FeIter<FeInt> = &fe_list![FeInt::from(1), FeInt::from(2), FeInt::from(3)];

let mut values = fe_list![];

for i in FeInt::from(0)..FeInt::from(10) {
    values.push(i);
}

print(values);
```

## `[...; N]` - Arrays

`[]` can also be used to create arrays, when a size is specified.

Ferrum:
```rust
const values: [int; 3] = [1, 2, 3]

fn print_all(numbers: &[int; N])
    print("Printing {N} numbers:")

    for number in numbers
        print(number)
    ;
;

print_all(&values)
print_all(&[])

const values: [int; _] = [2 * x for x in 1..=3]

print_all(&values)
```

Rust:
```rust
fn print_all<const N: usize>(numbers: &[FeInt; N]) {
    print(format!("Printing {} numbers:", N));

    for number in numbers {
        print(number);
    }
}

fn main() {
    let values: [FeInt; 3] = [FeInt::from(1), FeInt::from(2), FeInt::from(3)];

    print_all(&values);
    print_all(&[]);

    let values: [FeInt; 3] = [
        {
            let x = 1;
            2 * x
        },
        {
            let x = 2;
            2 * x
        },
        {
            let x = 3;
            2 * x
        },
    ];

    print_all(&values);
}
```

## `#{ ... }` - Maps

`#{}` can be used to create dynamic key-value maps.

Ferrum:
```rust
const values: #{ char: int } = #{
    'a': 1,
    'b': 2,
    'c': 3,
}

// The above is syntactic-sugar for:
const values: Map<char, int> = Map(
    ('a', 1),
    ('b', 2),
    ('c', 3),
)

const my_char = 'z'

const my_int = 123

let mapping = #{
    my_char: my_int,
}

if mapping.get(&my_char) is Some(n)
    print(n)
;

mapping['x'] = 0

```

Rust:
```rust
let values: FeMap<char, FeInt> = fe_map![
    ('a', 1),
    ('b', 2),
    ('c', 3),
];

let values: FeMap<char, FeInt> = fe_map![
    ('a', 1),
    ('b', 2),
    ('c', 3),
];

let my_char = 'z';

let my_int = FeInt::from(123);

let mut mapping = fe_map![
    (my_char, my_int),
];

if let Some(n) = mapping.get_value(&my_char) {
    print(n);
}

mapping.insert('x', FeInt::from(0));
```

## `#[]` - Sets

`#[]` can be used to create sets.

Ferrum:
```rust
const values: #[int] = #[1, 1, 2, 2, 3]

// The above is syntactic-sugar for:
const values: Set<int> = Set(1, 1, 2, 2, 3)

print(values.len()) // 3

const my_int = 42

let set = #[my_int]

if set.has(&my_int)
    print(true)
;

set.insert(101)
```

Rust:
```rust
let values: FeSet<FeInt> = fe_set![1, 1, 2, 2, 3];

let values: FeSet<FeInt> = fe_set![1, 1, 2, 2, 3];

print(values.len());

let my_int = FeInt::from(42);

let mut set = fe_set![my_int];

if set.get(&my_int) {
    print(true);
}

set.insert(FeInt::from(101));
```

## `..` - Spread

`..` syntax can be used to auto-fill the rest of something, when it is obvious to the compiler.

Ferrum:
```rust
const (a, b, .., f, g) = (1, 2, 3, 4, 5, 6, 7)

print(a, b, f, g) // 1, 2, 6, 7

if [1, 2, 3, 4] is [.., 3, n]
    print("ends with 3, {n}")
;

struct Unique<T> {
    inner_id: UUID = UUID(),
    value: T,
} impl
    pub construct(value) ..
;
```

Rust:
```rust
struct Unique<T> {
    inner_id: FeUUID,
    value: T,
}

impl<T> Unique<T> {
    pub fn new(value: T) -> Self {
        return Self {
            value,
            inner_id: FeUUID::new(),
        };
    }
}

fn main() {
    let (a, b, f, g) = {
        let values = (1, 2, 3, 4, 5, 6, 7);
        (values.0, values.1, values.5, values.6)
    };

    print(format!("{}, {}, {}, {}", a, b, f, g));

    if let Some((3, n)) = {
        let values = fe_list![1, 2, 3, 4];

        if values.len() < 2 {
            None
        } else {
            Some((values[values.len() - 2], values[values.len() - 1]))
        }
    } {
        print(format!("ends with 3, {}", n))
    }
}

```

## `stable` - Safe functions

Safe functions cannot contain any code paths that cause a panic

Note: the `stable` keyword can be ignored when running a development build

Ferrum:
```rust
pub stable fn main()
    print("Can't panic!")

    // won't compile as stable fns can't panic, and some_func contains a panic
    // some_func()
;

fn some_func()
    #TODO
;
```

## `~` - Dynamic Interface Objects

Interfaces can be referenced directly (using monomorphization), or dynamically with `~` (using dynamic dispatch)

Ferrum:
```rust
interface Connection
    &mut self.connect() -> bool
;

struct DbConnection impl Connection
    pub &mut self.connect() -> bool
        return true
    ;
;

struct Wrapper1<T: Connection> {
    c: &mut T,
}

struct Wrapper2 {
    c: &mut ~Connection,
}

fn handle_connection1<T: Connection>(c: &mut T)
    let wrapper = Wrapper1 { c }
    wrapper.c.connect()
;

fn handle_connection2(c: &mut ~Connection)
    let wrapper = Wrapper2 { c }
    wrapper.c.connect()
;

let y: DbConnection = DbConnection()
handle_connection1(&mut y)

let y: DbConnection = DbConnection()
handle_connection2(&mut y)

// ~Connection cannot be passed as Connection
// let y: ~Connection = DbConnection()
// handle_connection1(&mut y)

let y: ~Connection = DbConnection()
handle_connection2(&mut y)
```

Rust:
```rust
trait Connection {
    fn connect(&mut self) -> bool;
}

struct DbConnection;
impl Connection for DbConnection {
    fn connect(&mut self) -> bool {
        return true;
    }
}

struct Wrapper1<'a, T: Connection> {
    c: &'a mut T,
}

struct Wrapper2<'a> {
    c: &'a mut dyn Connection,
}

fn handle_connection1(c: &mut impl Connection) {
    let mut wrapper = Wrapper1 { c };
    wrapper.c.connect();
}

fn handle_connection2(c: &mut dyn Connection) {
    let mut wrapper = Wrapper2 { c };
    wrapper.c.connect();
}

// let mut y: Box<dyn Connection> = Box::new(DbConnection);
// handle_connection1(&mut *y);

let mut y: DbConnection = DbConnection;
handle_connection1(&mut y);

let mut y: Box<dyn Connection> = Box::new(DbConnection);
handle_connection2(&mut *y);

let mut y: DbConnection = DbConnection;
handle_connection2(&mut y);
```

## `=>` - Closures

`=>` syntax can be used to create closures.

Ferrum:
```rust
const x = 1
const add: ~Fn(int, int) -> int = (a, b) => a + b + x

let y = 2
let add_mut: ~FnMut(int, int) -> int = (a, b) => do
    y += a + b
    yield y
;

const z = 3
const add_once: ~FnOnce(int, int) -> int
    = (a, b) => a + b + z

const value1 = add(3, 4)
const value2 = add_mut(3, 4)
const value3 = add_once(3, 4)
```

Rust:
```rust
let x = FeInt::from(1);
let add: Box<dyn Fn(FeInt, FeInt) -> FeInt> = Box::new(|a, b| a + b + x);

let mut y = FeInt::from(2);
let mut add_mut: Box<dyn FnMut(FeInt, FeInt) -> FeInt> = Box::new(|a, b| {
    y += a + b;
    return y;
});

let z = FeInt::from(3);
let add_once: Box<dyn FnOnce(FeInt, FeInt) -> FeInt> = Box::new(|a, b| {
    return a + b + z;
});

let value1 = add(3, 4);
let value2 = add_mut(3, 4);
let value3 = add_once(3, 4);
```

## `use ...` - Use statements

`use` can be used to import and/or re-export

Ferrum:
```rust
/*
Given:
src
|- _main.fe
|- foobar.fe
|- utils
|  |- _pkg.fe
|  |- inner.fe
|  |- strings.fe
*/

// src/_main.fe
    use ./utils // `./` points to current dir
    use ~/utils // `~/` points to src dir

    use utils::strings
    
    // Can't access utils/inner
    // use utils::inner


// src/utils/_pkg.fe
    pub use strings
    use inner // not pub


// src/utils/inner.fe
    use ../foobar // Can access siblings of any parent
```

Rust:
```rust
// src/main.rs
    pub mod utils;
    pub mod foobar;

    use utils;
    use crate::utils;

    use utils::strings;


// src/utils/mod.rs
    pub use strings;
    use inner;

// src/utils/inner.rs
    use super::super::foobar;
```

---

# Example Ferrum Program:
```rust
use std::prelude::*
use std::cli

print("S-Expression Calculator!")
print("Enter an S-Expression containing integers, 'add', and/or 'multiply' to see the result:")

let cache = Cache()

loop
    const input = cli::await_line()!
        .then_mut_trim()

    match parse_s_expression(input, &mut cache)
        Ok(result) => print("Result: {result}"),
        Err(message) => print_err("Error! {message}"),
    ;
;

alias Cache = Map<String, int>

pub stable fn parse_s_expression(
    text: String,
    cache: &mut Cache = &mut Cache()
) -> !int
    const NO_OPEN_MSG = "No opening '(' was found to match the closing ')'."

    // Note: Closures that don't use surrounding context are optimized into functions
    const build_invalid_int_msg = (val) => "Invalid integer: {val}"
    const build_unrecognized_msg = (val) => "Unrecognized S-Expression format: {val}"

    while text.find_index(')') is Some(close_paren_idx)
        if close_paren_idx is 0
            // Won't really clone NO_OPEN_MSG here. Under the hood it will share the string.
            // Clone only happens if string is non-static
            return Err(clone(&NO_OPEN_MSG))
        ;

        if cache.get(&text)
            return @it
        ;

        const open_paren_idx = text.find_index(
            '(',
            reverse = true,
            
            // Note: Compiler knows that (close_paren_idx - 1) can safely be inferred as NonNegative<int> here
            //       because close_paren_idx can safely be inferred as NonZero<int> here
            //       This means coersion to uint is safely inferred here
            max = close_paren_idx - 1,
        )

        const open_paren_idx = open_paren_idx ?? return Err(clone(&NO_OPEN_MSG))

        const simple_expr = &text[open_paren_idx + 1 .. close_paren_idx]

        if cache.get(simple_expr)
            return @it
        ;

        const value = match simple_expr.split(' ')
            ["add" or "multiply" as cmd, a, b] => do
                const a = a.parse::<int>() ?? return Err(build_invalid_int_msg(a))
                const b = b.parse::<int>() ?? return Err(build_invalid_int_msg(b))

                yield match cmd
                    "add" => a + b,
                    "multiply" => a * b,
                ;
            ;

            [value] if value.parse::<int>() is Ok(value) => value,

            _ => return Err(build_unrecognized_msg(simple_expr))
        ;

        cache.insert(clone(simple_expr), value)

        if open_paren_idx is 0
            return value
        else
            text.replace_range(open_paren_idx ..= close_paren_idx, value)
        ;
    ;

    return text.parse()
;
```

Pure Rust version:
```rust
use std::collections::HashMap;
use std::io;

fn main() -> Result<(), String> {
    println!("S-Expression Calculator!");
    println!("Enter an S-Expression containing integers, 'add', and/or 'multiply' to see the result:");

    let mut cache = Cache::new();

    let cli = io::stdin();

    loop {
        let mut input = String::new();
        cli
            .read_line(&mut input)
            .map_err(|e| format!("{e}"))?;

        let input = input.trim().to_string();

        match parse_s_expression(input, Some(&mut cache)) {
            Ok(result) => println!("Result: {result}"),
            Err(message) => println!("Error! {message}"),
        }
    }
}

type Cache = HashMap<String, isize>;

fn parse_s_expression(
    mut text: String,
    cache: Option<&mut Cache>,
) -> Result<isize, String> {
    let cache = {
        use std::mem::MaybeUninit;

        let mut created_cache: MaybeUninit<Cache> = MaybeUninit::zeroed();
        let created_cache_mut_ref: &mut Cache = unsafe { &mut *created_cache.as_mut_ptr() };

        &mut match cache {
            Some(cache) => cache,
            None => {
                created_cache.write(Cache::new());
                *created_cache_mut_ref = unsafe { created_cache.assume_init() };

                created_cache_mut_ref
            },
        }
    };
    
    const NO_OPEN_MSG: &'static str = "No opening '(' was found to match the closing ')'.";

    let build_invalid_int_msg = |val: String| -> String { format!("Invalid integer: {val}") };
    let build_unrecognized_msg = |val: String| -> String { format!("Unrecognized S-Expression format: {val}") };

    while let Some(close_paren_idx) = text.find(')') {
        if close_paren_idx == 0 {
            return Err(String::from(NO_OPEN_MSG));
        }

        if let Some(cached) = cache.get(&text) {
            return Ok(*cached);
        }

        let open_paren_idx = text[..close_paren_idx].rfind(
            '(',
        );

        let Some(open_paren_idx) = open_paren_idx else {
            return Err(String::from(NO_OPEN_MSG));
        };
        let open_paren_idx = if let Some(open_paren_idx) = open_paren_idx {
            open_paren_idx
        } else {
            return Err(String::from(NO_OPEN_MSG));
        };

        let simple_expr = &text[open_paren_idx + 1 .. close_paren_idx];

        if let Some(cached) = cache.get(simple_expr) {
            return Ok(*cached);
        }

        let value: isize = match simple_expr.split(' ').collect::<Vec<&str>>()[..] {
            [cmd @ ("add" | "multiply"), a, b] => {
                let Ok(a) = a.parse::<isize>() else { return Err(build_invalid_int_msg(a.to_string())) };
                let Ok(b) = b.parse::<isize>() else { return Err(build_invalid_int_msg(b.to_string())) };

                match cmd {
                    "add" => a + b,
                    "multiply" => a * b,
                    _ => unreachable!(),
                }
            }

            [value] => if let Ok(value) = value.parse::<isize>() { value } else {
                return Err(build_unrecognized_msg(String::from(simple_expr)));
            },

            _ => return Err(build_unrecognized_msg(String::from(simple_expr))),
        };

        cache.insert(String::from(simple_expr), value);

        if open_paren_idx == 0 {
            return Ok(value);
        } else {
            text.replace_range(
                open_paren_idx ..= close_paren_idx,
                &value.to_string(),
            );
        }
    }

    return text.parse().map_err(|_| build_invalid_int_msg(text));
}
```

idea:

```rust
struct String {
    chars: Chars,
}

impl
    // `return self`, NOT `return Self`
    // Difference being that return self will return with whatever type the method was called on
    // So if owned type, mutable reference will happen automatically, and the owned value will be returned
    pub &mut self.trim() -> self
        let start_count = 0
        let end_count = 0
        
        for c in self
            if c.is_whitespace()
                start_count += 1
            else
                break        
            ;
        ;

        for c in self.reverse()
            if c.is_whitespace()
                end_count += 1
            else
                break
            ;
        ;

        self.replace_range(
            self.len() - end_count .. self.len(), // this is optimized to 1 len() call (or maybe just inlined)
            '',
        )

        self.replace_range(
            0 ..= start_count,
            '',
        )
    
        // return not needed for `self`
    ;

    pub &self.then_print() -> self
        print(self)
    ;
;

const trimmed_str: String = "   abc   123   ".trim()

// The rust equivalent would be:
let trimmed_str: String = {
    let mut tmp = String::from("   abc   123   ");
    tmp.trim();
    tmp
};

const x = "  abc  123  ".clone().trim()
```


```rust
use std::prelude::*

const (rx_channel, tx_channel) = Channel()

let tasks = AsyncTasks()

tasks.start_soon(() => do
    let tx = tx_channel.share()

    const res = http::get("www.google.com")

    match res
        Ok(res) => tx.send(res)
        Err(e) => print_err(e)
    ;
;)

tasks.start_soon(() => do
    let tx = tx_channel.share()

    const res = http::get("https://example.com")

    match res
        Ok(res) => tx.send(res)
        Err(e) => print_err(e)
    ;
;)

const rx = rx_channel.share()

while rx.await_next(timeout_ms = 1000) is Ok(res)
    print(res)
;

tasks.await_all(timeout_ms = 50)!
```

