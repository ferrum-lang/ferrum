# The Ferrum Programming Language

## Language Syntax

### `const`

Immutable declaration.

Ferrum:
```rust
const x = true
```

Rust:
```rust
let x = true;
```

### `let`

Mutable declaration.

Ferrum:
```rust
let x = true
```

Rust:
```rust
let mut x = true;
```

### `pub`

Ferrum:
```rust
pub use ./utils

pub struct A { }
pub fn hello() { }
```

Rust:
```rust
pub use super::utils;

pub struct A { }
pub fn hello() { }
```

### `struct`

Just Rust structs but simplified so that serialization, cloning, debug formatting, etc are built in.

Ferrum:
```rust
struct A {
    field: String,
}
```

Rust:
```rust
struct A {
    field: String,
}
```

### `impl`

Adding function and method implementations for a struct.

Ferrum:
```rust
struct Stack<T> {
    values: [T] = [],
}

impl
    pub fn from_list(list: [T]) -> Self
        return Self(list)
    ;

    pub &mut self.push(value: T)
        self.values.push(value)
    ;
;
```

Rust:
```rust
struct Stack<T> {
    values: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(values: Option<Vec<T>>) -> Self {
        return Self {
            values: values.unwrap_or_else(|| vec![]),
        };
    }

    pub fn from_list(list: Vec<T>) -> Self {
        return Self::new(Some(list));
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);
    }
}
```

### `enum`

Enums are just Rust enums, but with some additional available features like values, ordinals, etc.

Ferrum:
```rust
enum WeekDay {
    Monday = "MON",
    Tuesday = "TUES",
    Wednesday = "WED",
    Thursday = "THURS",
    Friday = "FRI",
}
```

Rust:
```rust
enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl Weekday {
    pub fn value(&self) -> String {
        return match self {
            Self::Monday => String::from("MON"),
            Self::Tuesday => String::from("TUES"),
            Self::Wednesday => String::from("WED"),
            Self::Thursday => String::from("THURS"),
            Self::Friday => String::from("FRI"),
        };
    }

    pub fn ordinal(&self) -> usize {
        return match self {
            Self::Monday => 0,
            Self::Tuesday => 1,
            Self::Wednesday => 2,
            Self::Thursday => 3,
            Self::Friday => 4,
        };
    }

    // + more included methods & functions ...
}
```

### `trait`

Trait implementations must be either after the struct definition, or the trait definition.

Ferrum:
```rust
struct Person { }


trait CanSpeak
    fn &self.speak() -> String
;

impl for Person
    fn &self.speak() -> String
        return "Hello"
    ;
;


struct Ghost { }

impl CanSpeak
    fn &self.speak() -> String
        return "Boo!"
    ;
;
```

Rust:
```rust
struct Person { }


trait CanSpeak {
    fn speak(&self) -> String;
}

impl CanSpeak for Person {
    fn speak(&self) -> String {
        return String::from("Hello");
    }
}


struct Ghost { }

impl CanSpeak for Ghost {
    fn speak(&self) -> String {
        return String::from("Boo!");
    }
}
```

### `newtype`

Type wrapping for newtype pattern.

Ferrum:
```rust
newtype Name = String
```

Rust:
```rust
struct Name(String);

impl From<String> for Name {
    fn from(value: String) -> Self {
        return Self(value);
    }
}

impl From<Name> for String {
    fn from(value: Name) -> Self {
        return value.0;
    }
}

impl std::ops::Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl std::ops::DerefMut for Name {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
```

### `alias`

Type aliasing to make development easier

Ferrum:
```rust
alias Cache = Map<String, uint>
```

Rust:
```rust
type Cache = Map<String, usize>;
```

### `self`

References the struct in a method implementation.

Ferrum:
```rust
const apple = A()

apple.hello()

struct A impl
    &self.hello()
        // self here would represent `apple`
    ;
;
```

Rust:
```rust
fn main() {
    let apple = A::new();

    apple.hello();
}

struct A;

impl A {
    pub fn new() -> Self {
        return Self;
    }

    fn hello(&self) {
        // self here would represent `apple`
    }
}
```

### `mut`

Sets requirement for type to be mutable.

Ferrum:
```rust
let x_owned: String = "Hello"

let x_mut_borrowed: &mut String = &mut x_owned

x_mut_borrowed += " World"
```

Rust:
```rust
let mut x_owned: String = String::from("Hello");

let mut x_mut_borrowed: &mut String = &mut x_owned;

x_mut_borrowed.append_string(String::from(" World"));
```

### `require`

Require methods to be called before a struct is dropped.

Ferrum:
```rust
struct AsyncTasks { /* ... */ }

impl
    require pub self.await_all()
        /* ... */
    ;
;

let tasks = AsyncTasks()

// This line is required for program to compile
tasks.await_all()
```

### `fn`

Just Rust functions, but with different syntax, and optionals & defaults built-in.

Ferrum:
```rust
fn process(
    data: &mut [int],
    config: Config = Config::default(),
    debug_logger: ?Logger,
)
    /* ... */
;

process(&mut [1, 2, 3])

process(
    debug_logger = fetch_debug_logger(),
    config = build_config(),
    data = &mut [],
)
```

Rust:
```rust
fn process(
    data: &mut Vec<isize>,
    config: Option<Config>,
    debug_logger: Option<Logger>,
) {
    let config = config.unwrap_or_else(|| Config::default());

    /* ... */
}

fn main() {
    {
        let mut temp_data = vec![1, 2, 3];
        process(&mut temp_data, None, None)
    };

    {
        let mut temp_data = vec![1, 2, 3];
        process(&mut temp_data, Some(build_config()), Some(fetch_debug_logger()))
    };
}
```

### `use`

### `do`

### `yield`

### `if`

### `it`

### `is`

### `else`

### Docs TODO:
```
use
do
yield
if
it
is
else
for
in
while
loop
return
break
match
```
