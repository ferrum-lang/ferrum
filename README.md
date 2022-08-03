# The Ferrum Programming Language

## Important! This language is a prototype-in-progress. The ideas & repos are very much in early conception, and not yet finished or fleshed out.

Meant as a "wrapper" or "layer" over-top of Rust. More accessible, faster development, just as safe (if not safer), but less powerful, and less performant.

## The Oxidize Build Tool

This project is the build tool called `oxidize`, designed to build and run the Ferrum programming language. Under-the-hood, the input code written in Ferrum gets transpiled to Rust code, then the Rust code is built and run.

## Why?

Rust is a fantastic programming language that changes the status-quo. But it's a self-proclaimed "systems" programming language, designed for low-level programming including interacting with hardware, full memory control, unsafe code, and many other features that are unnecessary for most non-systems programming.

The goal of this language is to take the lessons from Rust, and apply them to a higher-level "general-purpose" programming language that is built on Rust "under-the-hood". Concepts like managing mutability, ownership & borrowing, compile-time match guarantees, and more. But also without ever worrying about "lifetimes"; making unique / shared memory easy with opt-in automatic reference counting; a single, easy to use, string type; string templating; variable arguments; dynamic lists instead of tricky arrays; and much more!

Of course, building some of these concepts means losing some performance. But for the average programmer, the lose in performance should really be minimal compared to the gain in accessibility. We will also allow interopability with Rust in case high-performance or unsafe code is necessary for a portion of your applications. And in the end, we're running on Rust. It's a highly-optimized, compiled language with no garbage collector and a minimal overall runtime footprint. Performance will be significantly better than what you'll find in languages like Python, Javascript, Ruby, Java, C#, etc.

## Keywords

###### Note: This list is a work-in-progress

- `let` - Declares a new variable that is allowed to be reassigned

```
let x = 123;
x = 456;
```

- `const` - Declares a new variable that is **not** allowed to be reassigned

```
const x = 123;
```

- `*`

- `""`

- `[]`

- `()`

- `..`

- `if`

- `not`

- `else`

- `true`

- `false`

- `match`

- `matches`

- `loop`

- `while`

- `for`

- `struct`

- `class`

- `interface`

- `enum`

- `alias`

- `state`

- `fn`

- `mut`

- `some`

- `none`

- `ok`

- `err`

- `PANIC`

- `TODO`

- `UNREACHABLE`

- `=`

- `==`

- `!=`

- `~=`

### Built-in types

- `bool`

- `bit`

- `uint8`

- `byte`

- `uint16`

- `uint32`

- `uint64`

- `uint128`

- `uint`

- `biguint`

- `int8`

- `int16`

- `int32`

- `int64`

- `int128`

- `int`

- `bigint`

- `float32`

- `float64`

- `float`

- `bigfloat`

- `char`

- `string`

- `[]`

- `()`

- `?`

- `!`

###### Created by Adam Bates
