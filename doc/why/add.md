# The "Add" trait for String

Why does the code below produces a compilation error?

```rust
use std::ops::Add;

fn tester() -> String {
    let s1: String = String::from("abc");
    let s2: String = String::from("def");
    let s3: String = s1.add(s2.as_str());
    println!("{}", s1); // This will generate an error.
    return s3;
}

fn main() {
    let _result = tester();
}
```

Error:

```bash
$ cargo build
   Compiling variables v0.1.0 (/home/denis/Documents/github/rust-playground/variables)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:7:20
  |
4 |     let s1: String = String::from("abc");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
5 |     let s2: String = String::from("def");
6 |     let s3: String = s1.add(s2.as_str());
  |                      -- value moved here
7 |     println!("{}", s1); // This will generate an error.
  |                    ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `variables` due to previous error
```

To understand this error, we need to look at the [description for the trait "Add"](https://doc.rust-lang.org/std/string/struct.String.html#impl-Add%3C%26str%3E-for-String).

`s1.add(s2.as_str())` <=> `s1 + s2.as_str()`
 
> This consumes the String on the left-hand side and re-uses its buffer (growing it if necessary).

Therefore, the value of the variable `s1` is used (to receive the value of `s2`). And, then, because 
String does not implement the Copy trait, the value of `s1` is moved to `s3`. Thus, `s1` does not 
hold a value anymore.

Please note that the value of `s2` is not moved. Therefore, the code below is valid:

```rust
use std::ops::Add;

fn tester() -> String {
    let s1: String = String::from("abc");
    let s2: String = String::from("def");
    let s3: String = s1.add(s2.as_str());
    println!("{}", s2);
    return s3;
}

fn main() {
    let _result = tester();
}
```

**The point is there is no way to understand the error unless we loot at the description of the Add trait.
And we cannot understand either if we are not aware that String does not implement the Copy trait.**
