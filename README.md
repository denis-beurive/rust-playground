# Rust playground

This repository contains notes about the Rust programming language.

# Installation notes

* [Installation and update notes for Windows](doc/installation-windows.md)
* [Installation and update notes for Linux](doc/installation-linux.md)
* [IDE configuration](doc/ide.md)

> Update the toolchain: `rustup update`

# Cargo

* Create a new project: `cargo new name_of_the_project`
* Compile: `cargo build`
* Run: `cargo run`
* Check that it compiles: `cargo check`
* Clean temporary files: `cargo clean`
* Configuration file: see [this link](https://doc.rust-lang.org/cargo/reference/config.html#hierarchical-structure).
  Under Linux, the default configuration file is: `$HOME/.cargo/config.toml` (which may not exist). 
* Compile a release: `cargo build --release`
* Compile for debugging: `cargo build --config "profile.dev.debug=true" --config "profile.dev.opt-level=0" --profile=dev` 
  (see [--config](https://doc.rust-lang.org/cargo/reference/config.html#command-line-overrides) option and [profile override](https://doc.rust-lang.org/cargo/reference/config.html#profile))
  
# Debugging

Instead of using `gdb`, you should use `rust-gdb`.

# Things to remember

> You can modify a variable only when it was initialized. 
> _It is obvious... but it is also better to remember it._

> Rust is all about static, compile time, analysis. _If Rust raises an error, 
> you can prefix the description of the error by "at compile time..."_

> Whenever we assign _something_ (must be a reference) to a reference, we "borrow". 
> A reference represents a borrow of some owned value.

> Rust usually focuses on object value (i.e. the interesting part of the contents)
> rather than object identity (memory addresses). See [this link](https://stackoverflow.com/questions/27852613/why-does-printing-a-pointer-print-the-same-thing-as-printing-the-dereferenced-po).

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

```rust
    let borrowed_string1: &str = "world";
    let borrowed_string2: &str = "world";
    // borrowed_string1 and borrowed_string2 reference the same data (at address 0x7ffd27113658).
    println!("{:p}", &borrowed_string1); // -> 0x7ffd27113658
    println!("{:p}", &borrowed_string2); // -> 0x7ffd27113658
```

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
* [What is str ?](doc/str.md)
* [String](doc/string.md)

# Must read

* [The slice type](https://doc.rust-lang.org/book/ch04-03-slices.html): don't assume that you can skip this document...

