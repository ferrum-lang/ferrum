# Language Rules

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
*y = "cba"
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

## `@` - Shared-Mutable State

The `@` symbol is a syntax that makes it easy to create shared mutable data using reference-counting.

Ferrum:
```
const name: string = "Adam"

// explicit creation
let adam: @<string> = @::new(name)

// Can use like any other data
let names: [@<string>] = [adam]

// Won't compile as @ implies mutable
// const adam = @::new(name)
```

Rust:
```rust
let name: FeStr = FeStr::from("Adam");

let adam: FeBox<FeStr> = FeBox::new(name);

let names: FeList<FeBox<FeStr>> = fe_list![adam];
```

Ferrum:
```
// coerced creation
let adam: @ = "Adam"

// sharing the data, not cloning
let adam2 = @adam

let adam3 = @adam

// shared-mutable state
adam.append('1')
adam2.append('2')

print(adam) // "Adam12"
print(adam2) // "Adam12"
print(adam3) // "Adam12"

print(@::count(adam)) // 3
```

Rust:
```rust
let mut adam: FeBox<_> = FeStr::from("Adam").into();
let mut adam2 = FeBox::share(adam.as_ref());
let mut adam3 = FeBox::share(adam.as_ref());

adam.append('1');
adam2.append('2');

print(adam);
print(adam2);
print(adam3);

print(FeBox::count(adam.as_ref()));
```

Ferrum:
```
let adam: @ = "Adam"

// Note: *@ creates a temporary mutable reference to avoid unnecessary RC-clones
*@adam = "Adam Bates"

// Won't compile, cannot take out of a mutable reference
// let taken = *@adam
```

Rust:
```rust
let mut adam: FeBox<_> = FeStr::from("Adam").into();

*(unsafe { FeBox::get_unsafe_mut(adam.as_ref()) }) = FeStr::from("Adam Bates");
```

## `fn` - Functions

Functions are defined using `fn`.

Notes:
- The main function is optional. Top-level statements are only supported in an entry file with no main function.
- Owned parameters are mutable by default
- Reference parameters have shallow immutability. Deep mutability depends on reference type (`&` vs `&mut`).

Ferrum:
```
fn main() {
    print("hello world")

    foo_bar()

    let name = get_name()

    say_hello(&name)

    const updated = update_name(&mut name)

    if updated {
        consume_name(name)
    }
}

fn foo_bar() {
    fn inner() {
        print("Hello from inner")
    }

    print("Hello from foo_bar")

    inner()
}

fn get_name() -> string {
    return "Adam"
}

fn say_hello(name: &string) {
    print("Hello, {name}")
}

fn update_name(name: &mut string) -> bool {
    if *name != "Adam" {
        return false
    }

    name.append(" Bates")

    return true
}

fn consume_name(name: string) {
    print("consumed: {name}")
}
```

Rust:
```rust
fn main() {
    print("hello world");

    foo_bar();

    let mut name = get_name();

    say_hello(&name);

    let updated = update_name(&mut name);

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

fn say_hello(name: &FeStr) {
    print(format!("Hello, {}", name));
}

fn update_name(name: &mut FeStr) -> bool {
    if name != FeStr::from("Adam") {
        return false;
    }

    name.append(FeStr::from(" Bates"));

    return true;
}

fn consume_name(mut name: FeStr) {
    print(format!("consumed: {}", name));
}
```

## `struct` - Structs

Create concrete data structures to hold data, and/or implement methods.

Ferrum:
```
primative Serial(uint)

struct Device(
    serial: Serial,
    is_active: bool = true,
)

struct Inventory(
    devices: Map<Serial, Device> = {},
) impl {
    &self.get_all() -> [Device] {
        return self.devices.values()
    }

    &mut self.add(device: Device) {
        self.devices.insert(device.serial, device);
    }

    self.decorate() -> Self {
        return Inventory(self.devices)
    }

    mut self.with_device(device: Device) -> Self {
        self.add(device)

        return self
    }
}

const inventory = Inventory()
```

Rust:
```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Serial(FeUint);

impl Serial {
    fn new(val0: FeUint) -> Self {
        return Self(val0);
    }
}

impl Copy for Serial {}

impl std::fmt::Display for Serial {
    fn fmt(&self, f: std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self.0);
    }
}

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

#[derive(Debug, Clone)]
struct Inventory {
    devices: FeMap<Serial, Device>,
}

impl Inventory {
    fn new(devices: Option<FeMap<Serial, Device>>) -> Self {
        return Self {
            devices: devices.unwrap_or_else(|| FeMap::new()),
        };
    }

    fn get_all(&self) -> FeList<Device> {
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
    MyStructVal(name: string, age: uint) = "some value",
}

const e: MyEnum = MyEnum::MyTupleVal(1, 'a', 'b')

if e.value() is some(value) {
    print(value)
}
```

Rust:
```rust

lazy_static! {
    ref VAL_2: FeStr = FeStr::from_slice("some_value");
}

enum MyEnum {
    MyEmptyVal,
    MyTupleVal(FeInt, char, char),
    MyStructVal { name: FeStr, name: FeUint },
}

impl MyEnum {
    pub fn value(&self) -> Option<&FeStr> {
        return match *self {
            Self::MyEmptyVal => None,
            Self::MyTupleVal(..) => None,
            Self::MyStructVal { .. } => Some(VAL_2),
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

## `&` - Shared References

Represent shared (immutable) references using the `&` syntax.

Ferrum:
```
const name = "Adam"

const borrow_name_1 = &name
const borrow_name_2 = &name

print(borrow_name_1, borrow_name_2)

fn get_largest_first_2(
    value1: -> &string, // -> & syntax tells us which references are used in return (helps w/ lifetimes)
    value2: -> &string,
    value3: &string,
) -> &string {
    print(value3)

    if value1.len() >= value2.len() {
        return value1
    } else {
        return value2
    }
}

const value1 = "a"
const value2 = "bb"

const largest
{
    const value3 = "hello"

    largest = get_largest_first_2(&value1, &value2, &value3)
}

// value3 has been dropped but this is still valid
print(largest)

struct HoldingRefs(&string, &[int]) impl {
    fn -> &self.get_values() -> (&string, &[int]) {
        return (&self.0, &self.1);
    }
}
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
    value1: -> &mut string, // -> & syntax tells us which references are used in return (helps w/ lifetimes)
    value2: -> &mut string,
    value3: &mut string,
) -> &mut string {
    print(value3)

    if value1.len() >= value2.len() {
        return value1
    } else {
        return value2
    }
}

let value1 = "a"
let value2 = "bb"

let largest
{
    let value3 = "hello"

    largest = get_largest_first_2(&mut value1, &mut value2, &mut value3)
}

// value3 has been dropped but this is still valid
print(largest)

struct HoldingRefs(&mut string, &mut [int]) impl {
    fn -> &mut self.get_values() -> (&mut string, &mut [int]) {
        return (&mut self.0, &mut self.1);
    }
}
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
const name: ?string = some("Adam")
const name: ?string = none

// coerced
const name: ?string = "Adam"

fn get_length(value: ?&string) -> ?uint {
    // ?. maps the optional
    const length: ?uint = value?.len()

    // ? short-circuts function, returning none if lhs is none
    const length: uint = length?

    // values can be coerced into some(...)
    return length
}

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

    let name: Option<FeStr> = FeStr::from("Adam").into();
    
    let length = get_length(name.as_ref());
}
```

## `!` - Result

Represent results using `!` syntax.

Ferrum:
```
const name: !string = ok("Adam")
const name: !string = err(123)

const name: !string = "Adam"

fn get_length(value: !&string) -> !uint {
    // !. maps the optional
    const length: !uint = value!.len()

    // ! short-circuts function, returning none if lhs is none
    const length: uint = length!

    // values can be coerced into ok(...)
    return length
}

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

    let name: FeResult<FeStr> = FeStr::from("Adam").into();
    
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

// Iter is an interface that defines iteration over a collect
const values: ~Iter<int> = [1, 2, 3]

// Unfortunately, this requires moving the data onto the heap.
// This can be avoided by using references
const values: &~Iter<int> = &[1, 2, 3]

// Lists can be mutated
let values = []

for i in 0..10 {
    values.push(i)
}

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

---

## Frontend Component syntax

Svelte-style, cleaned up, and using Ferrum

Ferrum Component:
```
prop name: &mut string = "World"
$: print("Hello, {name}")

<h1>Hello {name}</>
<input bind:value={&mut name} />
<>
    let count: @ = 0

    <h2>{count}</>
    <button on:click={() => *@count += 1} />
    <button on:click={() => *@count -= 1} />
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


