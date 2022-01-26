# Onership

## Ownership: move or not move ?

A value is "owned" by a variable. 

When a variable value is assigned to another variable (the target variable),
the ownership of the value _may be moved_ (to the target variable), _or not_.

* If the value can be copied, then it is copied to the target variable.
  Scalar values (integers...) can be copied. Objects can be copied if they 
  implement the method `Copy`. Please note that the class `String` does not 
  implement the method `Copy`.
* Otherwise, the ownership of the value is moved (to the target variable).

## Illustrations

### Use case 1: simple code

```rust
fn main() {
    // n1 and n2 are integers (scalars).
    let n1: i32 = 10;
    let n2 = n1;
    println!("n1: {}, &n1: {:p}", n1, &n1);
    println!("n2: {}, &n2: {:p}", n2, &n2); // the value of n1 is copied to n2.

    // s1 and s2 are instances of String.
    let s1: String = String::from("test");
    let s2 = s1; // The class String does not implement the method "Copy".
                 // Thus, the ownership of s1 value is moved to s2.
                 // Conclusion: s1 does not own any more value.
                 // Hence, you s1 is not usable.
    println!("s2: {}, &s2: {}", s1, &s1); // => error.
    println!("s1: {}, &s1: {}", s2, &s2);
}
```

If you try to compile:

```bash
$ cargo run
   Compiling memory-management v0.1.0 (/home/denis/Documents/github/rust-playground/memory-management)
error[E0382]: borrow of moved value: `s1`
  --> src/main.rs:14:33
   |
9  |     let s1: String = String::from("test");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
10 |     let s2 = s1; // The class String does not implement the method "Copy".
   |              -- value moved here
...
14 |     println!("s2: {}, &s2: {}", s1, &s1); // => error.
   |                                 ^^ value borrowed here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `memory-management` due to previous error
```

### Use case 2: using functions

The code below is valid. Please note the use of the method `clone`.
Cloning an instance of String is a way to explicitly copy it.

```rust
fn double_i32(x: i32) -> i32 {
    2 * x
}

fn double_string_value(x: String) -> String {
    // Warning: we clone the value referenced by x.
    x + " * 2"
}

fn double_string_ref(x: &String) -> String {
    // Warning: we clone the value referenced by x.
    x.clone() + " * 2"
}

fn main() {
    // The ownership of n1 value is not moved, since scalar value can be copied.
    let n1: i32 = 10;
    let n2 = double_i32(n1);
    println!("n1: {}", n1);
    println!("n2: {}", n2);

    // The class String does not implement the method "Copy".
    let s1 = String::from("100");
    let s2 = double_string_ref(&s1);
    let s3 = double_string_value(s1.clone());
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
}
```

The code below is not valid. The ownership of a string is moved to a function parameter.
But, after, the original sting owner is used. 

```rust
fn double_i32(x: i32) -> i32 {
    2 * x
}

fn double_string(x: String) -> String {
    x + " * 2"
}

fn main() {
    // The ownership of n1 value is not moved, since scalar value can be copied.
    let n1: i32 = 10;
    let n2 = double_i32(n1);
    println!("n1: {}", n1);
    println!("n2: {}", n2);

    // The ownership of s1 value is moved to the parameter of the function "double_string"
    // (since the class String does not implement the method "Copy").
    let s1 = String::from("100");
    let s2 = double_string(s1);
    println!("s1: {}", s1); // => error
    println!("s2: {}", s2);
}
```

If you try to compile:

```bash
$ cargo run
   Compiling memory-management v0.1.0 (/home/denis/Documents/github/rust-playground/memory-management)
error[E0382]: borrow of moved value: `s1`
  --> src/main.rs:20:24
   |
18 |     let s1 = String::from("100");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
19 |     let s2 = double_string(s1);
   |                            -- value moved here
20 |     println!("s1: {}", s1); // => error
   |                        ^^ value borrowed here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `memory-management` due to previous error
```