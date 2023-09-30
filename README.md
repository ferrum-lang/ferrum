# The Ferrum Programming Language

**Note:** This language is a work in progress. The designs and implementations are unstable.

## Another Programming Language?
Ferrum is meant to be a simple programming language, batteries-included, for general software development.

The syntax is simplified, the APIs are more constrained, and there are built-in standard solutions for many common problems.

Details of the language are not yet well documented. Sorry about that!

#### Some Key Features
- **Runtime panics are only possible from within `risk` functions.**
  - This makes it very easy to find problematic call-stacks when debugging runtime panics.
- **Simple basic types such as `Integer`, `Decimal`, and `String`.**
  - Ferrum's compiler will ensure that the most optimal types are used under-the-hood.
  - Type conversion (ie. `Integer` to `Decimal`) will happen implicitly whenever possible.
  - `Integer` supports ints of any size (if no constraints are set), and will fallback to a big-int if necessary
- **Compile-time type constraints.**
  - For example, `String<$Length=3>` can implicitly convert to `String<$MinLength=2, $MaxLength=5>`, but not the other way.
  - This encourages developers to do input validation as soon as possible, allowing the compiler to optimize as much as possible.
- **No need for `async`/`await`.**
  - Functions that should be async, will be. These functions will be awaited when called. There are separate mechanisms to deal with background tasks or awaiting multiple tasks.
- **Compile-time evaluation.**
  - Code that can be calculated at compile-time, will be.
- **`?` for optionals, `!` for results.**
  - `?String` is the type reference for an optional string
  - `!Integer` is the type reference for a result of either an int or some error
  - `!<Bool, String>` is the type reference for a result of either a bool or a string error
- **No lifetime syntax.**
  - Ferrum uses Rust's ownership model, but doesn't rely on Rust's lifetime syntax. Instead, sane defaults are used.
  - If multiple references are passed into a function, and a reference is returned from the function, then you can optionally specify a lifetime for the return.
    ```rust
    // The ' lifetime is optional here, but allows the msg reference to not be tied to a, b, and return-type
    fn largest(a: &'String, b: &'String, msg: &String): &'String
        print(msg)
        return if a > b then a else b
    ;
    ```
- **`$` for shared ownership.**
  - The compiler will do it's best to choose the best implementation for shared ownership. This may be a shared static-reference, reference-counting, or garbage collected.
  - The compiler will determine if atomic synchronized types are necessary or not.
    ```rust
    const shared_name_1: $String = $"Adam"        // Create shared-ownership object
    const shared_name_2: $String = $shared_name_1 // Immutably share ownership
    const shared_name_3: $String = $shared_name_1 // Immutably share ownership

    mut shared_name_1: $mut String = $mut "Adam"        // Create mutable shared-ownership object
    mut shared_name_2: $mut String = $mut shared_name_1 // Mutably share ownership
    const shared_name_3: $String = $shared_name_1       // Immutably share ownership
    ```
- **`#[]` for sets, `#{}` for maps.**
  - Sets are a list of unique items: `#[1, 2, 3, 3] == #[1, 2, 3] // true`
  - Maps are a map of unique keys to values: `#{ "one": 1, "one": 1 } == #{ "one": 1 } // true`

## Demo

### Hello World
```rust
use fe::print

fn main()
    print("Hello, World!")
;
```

For more examples, see [examples](https://github.com/ferrum-lang/ferrum/tree/main/examples)
