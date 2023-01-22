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

> You can modify a variable only when it was initialized (see [this link](https://stackoverflow.com/questions/53471996/why-does-the-rust-compiler-give-an-uninitialized-variable-error-when-initializin)). 
> _It is obvious... but it is also better to remember it._

> Rust is all about static, compile time, analysis. _If Rust raises an error, 
> you can prefix the description of the error by "at compile time..."_

> **borrow**:
>
> Whenever we assign _something_ (must be a reference) to a reference, we "borrow". A reference represents _a borrow_ of some owned value.
>
> The term "borrow" may designate one of these things :
>
> * (a) the action (to borrow something).
> * (b) **THE** reference to a variable (`&value`).
> * (c) **A** variable that contains **THE** reference to a variable (`let the_borrow = &value`). _You may have more than one borrow to a variable_.
>
> Please note that the term "borrow" may be a verb **or a noun** (see ["borrow definition"](https://www.dictionary.com/browse/borrow)).
>
> ```rust
> let value: u8 = 1;
> // "&value" is **THE** borrow of the variable "value."
> // "a_borrow" is **A** borrow of the variable "value."
> let a_borrow: &u8 = &value; // The borrow occurs here.
>
> // The function "the_borrower" borrows the value of the variable "value."
> the_borrower(&value); // The borrow occurs here.
> ```

> **lifetime**:
>
> The lifetime of a borrow (`let a_borrow = &value`) cannot outlive the borrowed value's (`value`) scope ([thanks to Quinedot](https://users.rust-lang.org/t/lifetime-and-generic-in-plain-english/87877/6)).
>
> ```rust
> // Does not compile
> {
>     let local = String::new(); // the borrowed value.
>     let a_borrow: &'static String = &local;
> } // local drops here
> ```
>
> But the lifetime of a borrow (`&value`) can be greater than the scope of the borrow (`let the_borrow = &value`) itself ([thanks to Quinedot](https://users.rust-lang.org/t/lifetime-and-generic-in-plain-english/87877/6)). Please note that, in this sentence, the term "borrow" designates "_**THE** borrow of a variable_," and "_**A** borrow of a variable_."
>
> ```rust
> fn example<'a>(string: &'a String) -> &'a String {
>    {
>       // "string" is a borrow (of some string passed as a parameter to the function).
>       // "_local" is (also) a "borrow".
>       let _local: &'a str = string;
>    } // the "scope" of (the "borrow) "_local" ends here.
>      // but the lifetime of the "borrow" "string" does not end here.
>
>    return string;
> }
> ```rust

> Rust usually focuses on object value (i.e. the interesting part of the contents)
> rather than object identity (memory addresses). See [this link](https://stackoverflow.com/questions/27852613/why-does-printing-a-pointer-print-the-same-thing-as-printing-the-dereferenced-po).

> Unlike the practices seen in other programming languages, modules definitions don't _necessarily_ rely on file organisation. "`mod module_name`" **creates** a module name "`module_name`": the module may be defined _inline_ ("`mod module_name { /* inline definition here */ }`") or through external files ("`mod module_name;`"). If "`some_file.rs`" has mod declarations in it, then the contents of the module files would be inserted in places where "mod" declarations in the crate file are found, before running the compiler over it ([source](https://doc.rust-lang.org/rust-by-example/crates.html)).



# Vocabulary

**Shadowing (a variable)**: a variable is redeclared.

```rust
    let v: i16 = 1;
    println!("v = {}", v);
    let v: i32 = 2; // Shadows the previous declaration.
    println!("v = {}", v);
```

**Inference**: Rust finds out the type of a variable (from the context).

**Ownership (of a value)**: A variable has ownership of a value. Or a variable "owns" a value. Or, again:
the value is "owned" by a variable.

**Transfer (or "move") of ownership (of a value)**: The ownership of a value may be transferred from a variable to 
another variable (if it cannot be implicitly copied). More [details](doc/ownership.md#ownership-movedtransferred-or-not-).
Values that can be implicitly copied implement the [Copy trait](https://doc.rust-lang.org/std/marker/trait.Copy.html)
(their types are called [copy types](https://dhghomon.github.io/easy_rust/Chapter_19.html)).
The [list of all "copy types"](https://doc.rust-lang.org/std/marker/trait.Copy.html#implementors).

**Copy type**: A type that implements the Copy trait. See [the list](https://doc.rust-lang.org/std/marker/trait.Copy.html#implementors).

**Borrow (a value)**: References allow you to refer to some value without taking ownership of it. With references we 
"borrow" the value of a variable. See [details](doc/ownership.md#references).

```rust
    let borrowed_string1: &str = "world";
    let borrowed_string2: &str = "world";
    // borrowed_string1 and borrowed_string2 reference the same data (at address 0x7ffd27113658).
    println!("{:p}", &borrowed_string1); // -> 0x7ffd27113658
    println!("{:p}", &borrowed_string2); // -> 0x7ffd27113658
```

* variables: own values.
* references: borrow values.

**Fat pointer**: this is a pointer with extra information. For example, "&str" is a fat pointer. 
It contains a pointer to the memory location that is the start of the string, and it also contains the length of the string.


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
* [String and str allocation](doc/string-str-alloc.md)
* [Modules](modules/README.md)
* [Arrays, slices, vectors of "boxed traits"](doc/boxed-traits.md)

# Why (does the compiler throw this error message) ?

* [The Add trait for string](doc/why/add.md)

# Must read

* [The slice type](https://doc.rust-lang.org/book/ch04-03-slices.html): don't assume that you can skip this document...

