# The Ferrum Programming Language

## Important!
This language is a prototype-in-progress. The ideas & repos are very much in early conception, and not yet finished or fleshed out._

## What is this?

This is a simplified, batteries-included rust-like programming language.

The goal of the language is to provide a trimmed-down version of Rust with strong opinions and a fully-featured std library for high-level software development.

## Language Design

Some notable differences between Ferrum and Rust:

- Ferrum's `stable` keyword guarantees at compile-time that there are no panicing code-paths
- Ferrum does not have `unsafe`
- Ferrum does not have macros
- Ferrum's main `fn` is optional
  - The entry file `./src/_main.fe` will allow top-level statements only if no main `fn` is defined
- Every Ferrum `fn` is capable of running concurrently
  - An async runtime (`tokio`) will be baked in to the binary only when required
- Iterations that can be run in parallel will be
  - A parallel executor (`rayon`) will be baked in to the binary only when required
- Ferrum makes optional garbage collection easy
  - A garbage collector (`shredder`) will be baked in to the binary only when required
- Ferrum code blocks don't use braces `{}`, instead:
  - Some syntax can open a code block, ie `for`-statements and `fn`s
  - Code blocks are closed with semicolons `;`
- Ferrum statements don't end with semicolons `;`, instead
  - New-lines have syntactic meaning, allowing new statements to be started
  - Note: A single statement can still span across multiple new-lines
- And many more smaller QoL features

Example program in Ferrum vs Rust

The following Ferrum code:
```rust
use std::prelude::*

use std::time::Duration

const MAX_SECS = 10
const TIMEOUT_MS = (MAX_SECS + 1) * 1000

// Run multiple asynchronous tasks concurrently,
// and in parallel if system has multiple threads
let tasks = AsyncTasks()

const finished_task_ids = Mutex([])

for secs, idx in [0, 2, 5, MAX_SECS]

    tasks.start_soon(() => do
        print("Task {idx}: Pre-sleep")

        sleep(Duration::from_secs(secs))

        print("Task {idx}: Post-sleep")

        let lock = finished_task_ids.await_lock()
        let list = lock.open()

        list.push(idx)
    ;)
;

tasks.await_all(timeout_ms = TIMEOUT_MS)!

print(finished_task_ids.into_inner())

// Structs look like Rust, but come with a constructor, Clone, Debug, PartialEq, Eq, etc...
struct Person {
    name: String,
    country: ?String,
}

const people = [
    Person("Adam Bates", "Canada"),
    Person("Stranger")
]


for person in people
    const hello = say_hello(
        question = &"How's it going?",
        &person,
    )

    print(hello)
;


fn say_hello(
    { name, country }: &Person,
    question: &String = &"How are you?",
) -> String
    let out = "Hello, my name is {name}"

    if country
        out += ", and I am from {country}"
    ;

    out += ". {question}"

    return out
;
```

Would output Rust code that looks something like:
```rust
use ferrum_runtime::lang as fe;
use ferrum_runtime::std::prelude::*;
use ferrum_runtime::std::time::Duration;

fn main() -> fe::Result<()> {
    const MAX_SECS: fe::UInt = fe::UInt::_10;
    const TIMEOUT_MS: fe::UInt = fe::UInt::_10_000;

    let mut tasks = AsyncTasks::new();

    let finished_task_ids = FeMutex::new(fe::list![]);

    for (idx, secs) in [fe::UInt::_0, fe::UInt::_2, fe::UInt::_5, MAX_SECS] {

        tasks.start_soon(async {
            print(fe::format!("Task {}: Pre-sleep", idx));

            sleep(Duration::from_secs(secs)).await;

            print(fe::format!("Task {}: Post-sleep", idx));

            let mut lock = finished_task_ids.await_lock().await;
            let mut list = lock.open();

            list.push(idx);
        });
    }

    tasks.await_all(fe::Some(TIMEOUT_MS)).await;

    print(fe::format!("{}", finished_task_ids.into_inner()));

    let people = fe::list![
        Person::new(FeString::from_static("Adam Bates"), fe::Some(FeString::from_static("Canada"))),
        Person::new(FeString::from_static("Stranger"), fe::None),
    ];

    for person in people {
        let hello = {
            let _question = FeString::from_static("How's it going?");

            say_hello(
                &person,
                &_question,
            )
        };

        print(hello);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Person {
    name: FeString,
    country: fe::Option<FeString>,
}

impl Person {
    pub fn new(name: FeString, country: fe::Option<FeString>) -> Self {
        return Self {
            name,
            country,
        };
    }
}

fn say_hello(
    person_: &Person,
    question_: fe::Option<&FeString>,
) -> FeString {
    let name = &person_.name;
    let country = &person_.country;
    
    let mut question_default_ = std::mem::MaybeUninit::<FeString>::zeroed();
    let question: &FeString = {
        match question_ {
            fe::Some(question) => question,
            fe::None => {
                question_default_.write(FeString::from_static("How are you?"));
                unsafe { question_default_.assume_init_ref() }
            },
        }
    };

    let mut out = fe::format!("Hello, my name is {}", name);

    if let Some(country) = country {
        out.append(fe::format!(", and I am from {}", country));
    }

    out.append(fe::format!(". {}", question));

    return out;
}
```
