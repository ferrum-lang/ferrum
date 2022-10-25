# The Ferrum Programming Language

## Important! This language is a prototype-in-progress. The ideas & repos are very much in early conception, and not yet finished or fleshed out.

At this stage, if you look too closely at any piece, you'll find problems. And that's ok :) Right now this project is just for fun.

The langauge is meant to be a "wrapper" or "layer" over-top of Rust, by transpiling into Rust source code and using Cargo to execute it.

What I hope to do with this, is a language with the type safety and style of Rust, but more accessible and faster development, limited scope (no `unsafe` or `macros`), and a minor performance hit.

## Language Design

The design of the language has been evolving as I come to understand the nuances of what I'm trying to do. I'll get an idea (for example, separating shallow vs deep mutability), and then hit a brick wall when it comes to propogating those changes out (ie. do function return values need to mark types as deeply mutable).

This is fun for me, but it means that anytime someone else looks at this language, it's probably different than the last time.

I've gotten to the point of transpiling some langauge code that calls functions and handles simple operations into Rust code, but it inevitably gets scrapped when I realize another major flaw in my design.

I'm currently on attempt 3, but not much development is happening as I'm still brainstorming solutions to solve the problems I've hit, while still staying true to my original goal of simplicity and accessibiliy.

Here's an example of what I envision the language to look like for a simple `nth_fibonnacci` recursive solution:

```
for n in 0..20
    print("{n}: {nth_fib(n)}")
;

fn nth_fib(n: uint, cache: &mut Cache = #{}) -> biguint
    type Cache = Map<uint, biguint>

    if n is (0 or 1)
        return n
    ;

    do return it if cache[n]

    const prev1 = nth_fib(n - 1, cache)
    const prev2 = nth_fib(n - 2, cache)

    const fib = prev1 + prev2

    cache[n] = fib

    return fib
;
```

## The Oxidize Build Tool

Ideally this project will just be a CLI that allows you to interface with the compiler, but the actual compiler I call `oxidize`, which will be a separate crate.

This way it should be easy to programmatically interface with the compiler for anyone who's interested.

## Why?

Rust is a fantastic programming language that changes the status-quo. But it's a self-proclaimed "systems" programming language, designed for low-level programming including interacting with hardware, full memory control, unsafe code, and many other features that are unnecessary for most non-systems programming.

The goal of this language is to take the lessons from Rust, and apply them to a higher-level "general-purpose" programming language that is built on Rust. Concepts like managing mutability, compile-time match guarantees, possibly ownership & borrowing, and more. But also without ever worrying about lifetimes; making unique / shared memory easy with opt-in automatic reference counting; a single, easy to use, string type; string templating; variable arguments; dynamic lists by default; and much more!

Of course, building some of these concepts means losing some performance. But for the average programmer, the loss in performance should be minimal compared to the gain in accessibility.


