# Ownership / transfer / borrow

## Variables and ownership of values

A value is owned by a variable <=> A variable has ownership of a value.

## Transfer of ownership of value

A value _may be_ transferred (_if it cannot be implicitly copied_) from a variable to another variable
<=> the ownership of a value _may be_ transferred from a variable to another variable.

## References 

References allow you to refer to some value **without taking ownership** of it.
With references we "borrow" the value of a variable.

> Please note that: at any given time, you can have either one mutable reference or any number of immutable references 
> (see [the rules of references](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html#the-rules-of-references)).

The [code](src/ownership2.rs) below illustrates the process of borrowing the value of a variable:

```rust
fn main() {
    let mut s: String = String::from("10");
    // Here, we borrow the value owned by s1.
    let s_ref: &mut String = &mut s;
    s_ref.push_str(" * 2");
    // It is OK to use s1 since its value has not been transferred (only borrowed).
    println!("s: {}", s);
}
```

At any given time, you can have any number of immutable references.
The [code](src/ownership3.rs) below illustrates this:

```rust
fn main() {
    let s: String = String::from("10");
    // Here, we borrow the value owned by s1.
    // Please note that the references are not mutable (they cannot be used to change the borrowed value).
    let s_ref1: &String = &s;
    let s_ref2: &String = &s;

    // It is OK to use s1 since its value has not been transferred (only borrowed).
    println!("s: {}", s);
    println!("s_ref1: {}", s_ref1);
    println!("s_ref2: {}", s_ref2);
}
```

At any given time, relatively to a given variable, you can have either one mutable reference or
any number of immutable references.

> Thus, within a given scope, relatively to a given variable:
> 
> once you've declared an immutable reference, all previously declared mutable references are
> invalidated (_once and for all_).
>
> once you've declared a mutable reference, all previously declared immutable references are
> invalidated (_once and for all_).

The [code](src/ownership5.rs) below illustrates this:

```rust
fn main() {
  let mut s1: String = String::from("10");

  // ----------------------------------------------------------------------------
  // At any given time, you can have either one mutable reference or any number
  // of immutable references.
  // ----------------------------------------------------------------------------

  let s_ref1: &mut String = &mut s1;
  (|s: &String| { println!("s: {}", s); })(s_ref1);

  // We declare an immutable reference. But remember: at any given time, you
  // can have either one mutable reference or any number of immutable references.
  // Therefore, from this point, the previously mutable reference (s_ref1) becomes
  // invalid.

  let s_ref2: &String = &s1;
  (|s: &String| { println!("s: {}", s); })(s_ref2);
  // (|s: &mut String| { println!("s: {}", s); })(s_ref1); // This would generate an error.

  // OK, let define another mutable reference. But remember: at any given time,
  // you can have either one mutable reference or any number of immutable references.
  // Therefore, from this point, the previously immutable reference (s_ref2) becomes
  // invalid. Please note that s_ref1 has been invalidated (once and for all).
  let s_ref3: &mut String = &mut s1;
  (|s: &mut String| { println!("s: {}", s); })(s_ref3);
  (|s: &mut String| { println!("s: {}", s); })(&mut s1);
  // (|s: &String| { println!("s: {}", s); })(s_ref2); // This would generate an error.
}
```

## Ownership: moved/transferred or not ?

When a variable value is assigned to another variable (the target variable), the ownership of the value
_may be_ moved - or transferred - (to the target variable), **or not**.

* If the value can be implicitly copied, then it is copied to the target variable.
  Scalar values (integers...) can be copied. Objects can be copied if they 
  implement the method `Copy`. Please note that the class `String` does not 
  implement the method `Copy`.
* Otherwise, the ownership of the value is moved - or transferred - to the target variable.

> See the section "Assignment and ownership" of document [Variables and references](variables.md).

## Illustrations

### Use case 1: simple code

The [code](src/ownership1.rs) below illustrates the transfer of ownership.

```rust
fn main() {
  // n1 and n2 are integers (scalars).
  let n1: i32 = 10;
  let n2 = n1;
  // Because n1 and n2 values are scalars, the assignment of n1 to n2 will trigger the copy
  // of the value of n1 (that is: 10) to n2. Thus, there is no transfer of ownership of the value.
  // At the end, you get 2 values assigned to 2 variables.
  println!("n1: {}, &n1: {:p}", n1, &n1);
  println!("n2: {}, &n2: {:p}", n2, &n2); // the value of n1 is copied to n2.

  // s1 and s2 are instances of String.
  let s1: String = String::from("test");
  let s2 = s1;
  // The class String does not implement the method "Copy".
  // Thus, it is not possible to (implicitly) copy the value of s1 to s2.
  // Hence, the ownership of the value "test" is transferred from s1 to s2.
  // Conclusion: s1 does not own any more value.
  // Hence, s1 is not usable anymore.
  // At the end, you get one (and only one) value (that is: "test"). And this value is
  // owned by s2 (exclusively).
  println!("s2: {}, &s2: {}", s1, &s1); // => error (s1 does not own any value).
  println!("s1: {}, &s1: {}", s2, &s2);
}
```

If you try to compile:

```bash
$ cargo build
   Compiling variables v0.1.0 (/home/denis/Documents/github/rust-playground/variables)
error[E0382]: borrow of moved value: `s1`
  --> src/main.rs:21:33
   |
12 |     let s1: String = String::from("test");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
13 |     let s2 = s1;
   |              -- value moved here
...
21 |     println!("s2: {}, &s2: {}", s1, &s1); // => error (s1 does not own any value).
   |                                 ^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `variables` due to previous error
```

### Use case 2: using functions

The code below is valid. Please note the use of the method `clone`.
Cloning an instance of String is a way to explicitly copy it.

```rust
fn double_i32(x: i32) -> i32 {
    2 * x
}

fn double_string_value(x: String) -> String {
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
    let s3 = double_string_value(s1.clone()); // Please note the use of "clone()".
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