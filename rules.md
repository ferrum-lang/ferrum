# Some Language Rules
An incomplete collection of the language's rules and design decisions to help me keep track.

## `let` - Mutable Variable Declarations

Whenever `let` is used to declare a variable, it equates to `let mut` in Rust.

Note: Any normal declarations that have any mutability (shallow or deep) must use `let`

Ferrum:
```
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
let mut x = FeStr::from("abc");

let mut y = &x;
y = &FeStr::from("cba");

let mut z;

let mut y = &mut x;
*y = FeStr::from("cba");
y.append('!');
```

## `const` - Immutable Variable Declarations

Whenever `const` is used to declare a variable, it equates to `let` in Rust.

Note: `const` can only be used on non-mutable data, whether owned or shared reference

Ferrum:
```
const x = "abc"
const y = &x
const z

// The below line is invalid due to mutability:
// const y = &mut x
```

Rust:
```rust
let x = FeStr::from("abc");
let y = &x;
let z;
```

## `std::mem` - Smart Pointers

The `std::mem` package provides smart pointers for managing data in more complex manors.

Ferrum:
```
use std::mem::Rc

const name: Rc<str> = Rc::new("Adam")
const name2: Rc<str> = Rc::share(&name)

const name = Rc::new("Adam", lazy = true)   // Still on stack, not reference counting yet
const name2 = Rc::share(&name)              // Reference counting starts here
```

```
use std::mem::RcMut

let name: RcMut<str> = RcMut::new("Adam")   // lazy option available here too
let name2: RcMut<str> = RcMut::share(&name)
```

```
use std::mem::Gc

const name: Gc<str> = Gc::new("Adam")       // lazy option available here too
const name2: Gc<str> = Gc::share(&name)
```

```
use std::mem::GcMut

let name: GcMut<str> = GcMut::new("Adam")         // lazy option available here too
let name2: GcMut<str> = GcMut::share(&name)
```

Rust:
```rust
let name: FeStr = FeStr::from("Adam");

let mut adam: FeShared<FeStr> = FeShared::new(name);

let mut names: FeList<FeShared<FeStr>> = fe_list![adam];
```

Ferrum:
```
const name: str = "Adam"

// typical creation
let adam = @name

// sharing the data, not cloning
let adam2 = @adam

let adam3 = @adam

// shared-mutable state
adam.append('1')
adam2.append('2')

print(adam) // "Adam12"
print(adam2) // "Adam12"
print(adam3) // "Adam12"

print(@::count(&adam)) // 3
```

Rust:
```rust
let name: FeStr = FeStr::from("Adam");

let mut adam: FeShared<_> = FeShared::new(name);

let mut adam2 = FeShared::share(adam.as_ref());

let mut adam3 = FeShared::share(adam.as_ref());

adam.append('1');
adam2.append('2');

print(adam);
print(adam2);
print(adam3);

print(FeShared::count(&adam));
```

Ferrum:
```
// coerced creation
let adam: @ = "Adam"

// note: *@ creates a temporary mutable reference to avoid unnecessary RC-clones
*@adam = "Adam Bates"

// won't compile, cannot take out of a mutable reference
// let taken = *@adam
```

Rust:
```rust
let mut adam: FeShared<_> = FeShared::new(FeStr::from("Adam"));

*(unsafe { FeShared::get_unsafe_mut(adam.as_ref()) }) = FeStr::from("Adam Bates");
```

## `fn` - Functions

Functions are defined using `fn`.

Notes:
- The main function is optional. Top-level statements are only supported in an entry file with no main function.
- Owned parameters are mutable by default
- Reference parameters have shallow immutability. Deep mutability depends on reference type (`&` vs `&mut`).

Ferrum:
```
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

fn get_name() -> str
    return "Adam"
;

fn say_hello(name: &str, age: uint)
    print("Hello, {age} year old named {name}")
;

fn update_name(name: &mut str, force: bool = false) -> bool
    if !force && *name != "Adam"
        return false
    ;

    name.append(" Bates")

    return true
;

fn consume_name(name: str)
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

fn get_name() -> FeStr {
    return FeStr::from("Adam");
}

fn say_hello(name: &FeStr, age: FeUint) {
    print(format!("Hello, {} year old named {}", age, name));
}

fn update_name(name: &mut FeStr, force: Option<bool>) -> bool {
    let force = force.unwrap_or_else(|| false);

    if !force && name != FeStr::from("Adam") {
        return false;
    }

    name.append(FeStr::from(" Bates"));

    return true;
}

fn consume_name(mut name: FeStr) {
    name.append('!');

    print(format!("consumed: {}", name));
}
```

## `struct` - Structs

Create concrete data structures to hold data, and/or implement methods.

Ferrum:
```
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

## `enum` - Enums

Create enums.

Ferrum:
```
enum MyEnum {
    MyEmptyVal,
    MyTupleVal(int, char, char),
    MyStructVal {
        name: str,
        age: uint,
    } = "some value",
}

const e: MyEnum = MyEnum::MyTupleVal(1, 'a', 'b')

if e.value() is some(value)
    print(value)
;
```

Rust:
```rust

const VAL_2: FeStr = FeStr::from_slice("some_value");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MyEnum {
    MyEmptyVal,
    MyTupleVal(FeInt, char, char),
    MyStructVal { name: FeStr, name: FeUint },
}

impl MyEnum {
    pub fn value(&self) -> Option<&'static FeStr> {
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
```
alias Cache = Map<(char, char, int), ~Iter<@str>>

fn get_some() -> ?Map<(char, char, int), ~Iter<@str>>
    return none
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
type Cache = FeMap<(char, char, FeInt), Box<dyn FeIter<FeShared<FeStr>>>>;

fn get_some() -> Option<FeMap<(char, char, FeInt), Box<dyn FeIter<FeShared<FeStr>>>>> {
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
```
type Name = str
type Email = str
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

// won't compile as Name and Email are unique types, even though they both wrap str
// person.email = full_name
```

Rust:
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Name(FeStr);

impl<T: Into<FeStr>> From<T> for Name {
    fn from(value: T) -> Self {
        return Self(value.into());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Email(FeStr);

impl<T: Into<FeStr>> From<T> for FeStr {
    fn from(value: T) -> Self {
        return Self(value.into());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Age(FeUint);

impl Copy for Age {}

impl<T: Into<FeUint>> From<T> for FeStr {
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
    let name: Name = Name::from(FeStr::from("Adam"));

    let email = Email(FeStr::from("adam@example.com"));

    let age = Age(FeUint::from(101));

    let mut person = Person::new(name, email, age);

    let full_name: Name = Name::from(FeStr::from("Adam Bates"));
}
```

## `&` - Shared References

Represent shared (immutable) references using the `&` syntax.

Ferrum:
```
const name = "Adam"

const borrow_name_1 = &name
const borrow_name_2 = &name

print(borrow_name_1, borrow_name_2)

fn get_largest_first_2(
    value1: -> &str, // -> & syntax tells us which references are used in return (helps w/ lifetimes)
    value2: -> &str,
    value3: &str,
) -> &str
    print(value3)

    if value1.len() >= value2.len()
        return value1
    else
        return value2
    ;
;

const value1 = "a"
const value2 = "bb"

const largest = get
    const value3 = "hello"

    yield get_largest_first_2(&value1, &value2, &value3)
;

// value3 has been dropped but this is still valid
print(largest)

fn inferred(value: &str) -> &str
    return value
;

struct HoldingRefs(&str, &[int]) impl
    fn &self.get_values() -> (&str, &[int])
        return (&self.0, &self.1)
    ;
;
```

Rust:
```rust
fn get_largest_first_2<'a>(
    value1: &'a FeStr,
    value2: &'a FeStr,
    value3: &FeStr,
) -> &'a FeStr {
    print(value3);

    if value1.len() >= value2.len() {
        return value1;
    } else {
        return value2;
    }
}

fn inferred(value: &FeStr) -> &FeStr {
    return value;
}

struct HoldingRefs<'a, 'b>(&'a String, &'b Vec<isize>);

impl<'a> HoldingRefs<'a, 'a> {
    fn get_values(&'a self) -> (&'a String, &'a Vec<isize>) {
        return (&self.0, &self.1);
    }
}

fn main() {
    let name = FeStr::from("Adam");

    let borrow_name_1 = &name;
    let borrow_name_2 = &name;

    print(format!("{}, {}", borrow_name_1, borrow_name_2));

    let value1 = FeStr::from("a");
    let value2 = FeStr::from("bb");

    let largest;
    {
        let value3 = FeStr::from("hello");

        largest = get_largest_first_2(&value1, &value2, &value3);
    }

    print(largest);
}
```

## `&mut` - Mutable References

Represent mutable (unique) references using the `&mut` syntax.

Ferrum:
```
const name = "Adam"

// Won't compile: Cannot get mutable reference from a const
// let borrow_name_1 = &mut name

let name = name

let borrow_name_1 = &mut name

// Won't compile: cannot have multiple mutable references
// let borrow_name_2 = &mut name

print(borrow_name_1)

fn get_largest_first_2(
    value1: -> &mut str, // -> & syntax tells us which references are used in return (helps w/ lifetimes)
    value2: -> &mut str,
    value3: &mut str,
) -> &mut str
    print(value3)

    if value1.len() >= value2.len()
        return value1
    else
        return value2
    ;
;

let value1 = "a"
let value2 = "bb"

let largest = get
    let value3 = "hello"

    yield get_largest_first_2(&mut value1, &mut value2, &mut value3)
;

// value3 has been dropped but this is still valid
print(largest)

fn inferred(value: &str) -> &str
    return value
;

struct HoldingRefs(&mut str, &mut [int]) impl
    fn &mut self.get_values() -> (&mut str, &mut [int])
        return (&mut self.0, &mut self.1)
    ;
;
```

Rust:
```rust
fn get_largest_first_2<'a>(
    value1: &'a mut FeStr,
    value2: &'a mut FeStr,
    value3: &mut FeStr,
) -> &'a mut FeStr {
    print(value3);

    if value1.len() >= value2.len() {
        return value1;
    } else {
        return value2;
    }
}

fn inferred(value: &FeStr) -> &FeStr {
    return value;
}

struct HoldingRefs<'a, 'b>(&'a mut String, &'b mut Vec<isize>);

impl<'a> HoldingRefs<'a, 'a> {
    fn get_values(&'a mut self) -> (&'a mut String, &'a mut Vec<isize>) {
        return (&mut self.0, &mut self.1);
    }
}

fn main() {
    let name = FeStr::from("Adam");

    let mut name = name;

    let mut borrow_name_1 = &mut name;

    print(borrow_name_1);

    let mut value1 = FeStr::from("a");
    let mut value2 = FeStr::from("bb");

    let mut largest;
    {
        let mut value3 = FeStr::from("hello");

        largest = get_largest_first_2(&mut value1, &mut value2, &mut value3);
    }

    print(largest);
}
```

## `?` - Optional

Represent optional values using the `?` syntax.

Ferrum:
```
const name: ?str = some("Adam")
const name: ?str = none

// coerced
const name: ?str = "Adam"

fn get_length(value: ?&str) -> ?uint
    // ?. maps the optional
    const length: ?uint = value?.len()

    // ? short-circuts function, returning none if lhs is none
    const length: uint = length?

    // values can be coerced into some(...)
    return length
;

const length = get_length(&name)
```

Rust:
```rust
fn get_length(value: Option<&FeStr>) -> Option<usize> {
    let length: Option<usize> = value.map(|v| v.len());

    let length: usize = match length {
        Some(v) => v,
        None => return None,
    };

    return length.into();
}

fn main() {
    let name: Option<FeStr> = Some(FeStr::from("Adam"));
    let name: Option<FeStr> = None;

    let name: Option<FeStr> = Some(FeStr::from("Adam"));
    
    let length = get_length(name.as_ref());
}
```

## `!` - Result

Represent results using `!` syntax.

Ferrum:
```
const name: !str = ok("Adam")
const name: !str = err(123)

const name: !str = "Adam"

fn get_length(value: !&str) -> !uint
    // !. maps the optional
    const length: !uint = value!.len()

    // ! short-circuts function, returning none if lhs is none
    const length: uint = length!

    // values can be coerced into ok(...)
    return length
;

const length = get_length(&name)
```

Rust:
```rust
fn get_length(value: FeResult<&FeStr>) -> FeResult<usize> {
    let length: FeResult<usize> = value.map_ok(|v| v.len());

    let length: usize = length!;

    return length.into();
}

fn main() {
    let name: FeResult<FeStr> = Ok(FeStr::from("Adam"));
    let name: FeResult<FeStr> = Err(FeInt::from(123));

    let name: FeResult<FeStr> = Ok(FeStr::from("Adam"));
    
    let length = get_length(name.as_ref());
}
```

## `[...]` - Lists

`[]` can be used to create lists.

Ferrum:
```
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
```
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
```
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

if mapping[&my_char] is some(n)
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
```
const values: #[int] = #[1, 1, 2, 2, 3]

// The above is syntactic-sugar for:
const values: Set<int> = Set(1, 1, 2, 2, 3)

print(values.len()) // 3

const my_int = 42

let set = #[my_int]

if set[&my_int]
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
```
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
```
pub stable fn main() {
    print("Can't panic!")

    // won't compile as safe fns can't panic, and some_func contains a panic
    // some_func()
}

fn some_func() {
    #TODO
}
```

## `~` - Dynamic Contract Objects

Contracts can be referenced directly (using monomorphization), or dynamically with `~` (using dynamic dispatch)

Ferrum:
```
contract Connection
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

fn handle_connection1(c: &mut Connection)
    let wrapper = Wrapper1 { c }
    wrapper.c.connect()
;

fn handle_connection2(c: &mut ~Connection)
    let wrapper = Wrapper2 { c }
    wrapper.c.connect()
;

// ~Connection cannot be passed as Connection
// let y: ~Connection = DbConnection()
// handle_connection1(&mut y)

let y: DbConnection = DbConnection()
handle_connection1(&mut y)

let y: ~Connection = DbConnection()
handle_connection2(&mut y)

let y: DbConnection = DbConnection()
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
```
const x = 1
const add: ~Fn(int, int) -> int = a, b => a + b + x

let y = 2
let add_mut: ~FnMut(int, int) -> int = a, b => get
    y += a + b
    yield y
;

const z = 3
const add_once: ~FnOnce(int, int) -> int
    = a, b => a + b + z

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
```
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
    use utils
    use ~/utils // `~/` points to src root

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

## Frontend Component syntax

Svelte-style, cleaned up, and using Ferrum

Ferrum Component:
```
prop name: state<str> = "World"

$: print("Hello, {name}")

<h1>Hello {name}</>
<input bind:value={~name} />

<>
    let count: ~ = 0

    <h2>{count}</>
    <button value="+" on:click={() => @count += 1} />
    <button value="-" on:click={() => @count -= 1} />
</>
```

Svelte Component:
```svelte
<script>
	export let name = "World";
	$: console.log("Hello, %s", name);
	
	let count = 0;
</script>

<h1>Hello {name}!</h1>
<input bind:value={name} />

<div>
	<h2>{count}</h2>
	<button on:click={() => count += 1}>+</button>
	<button on:click={() => count -= 1}>-</button>
</div>
```


