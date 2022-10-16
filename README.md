# Rust playground

This repository contains notes about the Rust programming language.

# Installation notes

* [Installation and update notes for Windows](doc/installation-windows.md)
* [Installation and update notes for Linux](doc/installation-linux.md)
* [IDE configuration](doc/ide.md)

# Cargo

* Create a new project: `cargo new name_of_the_project`
* Compile: `cargo build`
* Run: `cargo run`
* Check that it compiles: `cargo check`
* Compile a release: `cargo build --release`

# Things to remember

> You can modify a variable only when it was initialized.
 
This is logical, isn't it ? 


# Vocabulary

**Shadowing (a variable)**: a variable is redeclared.

```rust
    let v: i16 = 1;
    println!("v = {}", v);
    let v: i32 = 2; // Shadows the previous declaration.
    println!("v = {}", v);
```

**Inference**: Rust finds out the type of a variable (from the context).

**Ownership (of a value)**: A variable has ownership of a value.

**Transfer (or "move") of ownership (of a value)**: The ownership of a value may be transferred from a variable to 
another variable (if it cannot be implicitly copied). More [details](doc/ownership.md#ownership-movedtransferred-or-not-).
Values that can be implicitly copied implement the [Copy trait](https://doc.rust-lang.org/std/marker/trait.Copy.html)
(their types are called [copy types](https://dhghomon.github.io/easy_rust/Chapter_19.html)).
The [list of all "copy types"](https://doc.rust-lang.org/std/marker/trait.Copy.html#implementors).

**Copy type**: A type that implements the Copy trait. See [the list](https://doc.rust-lang.org/std/marker/trait.Copy.html#implementors).

**Borrow (a value)**: References allow you to refer to some value without taking ownership of it. With references we "borrow" the value of a variable. See [details](doc/ownership.md#references).

# Notes

* [Variables and references](doc/variables.md)
* [Ownership](doc/ownership.md)
* [Loops](doc/loop.md)
* [Tuples](doc/tuple.md)
* [Arrays](doc/array.md)
* [Closure / anonymous functions](doc/closure.md)
* [Enums](doc/enum.md)
* [Match and guards](doc/match.md)
* [Option](doc/option.md)
* [String](doc/string.md)
