# Borrowing values

## Vocabulary

### Lifetimes and references

First let's clarify a point about the concept of lifetime:

Source: [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
* "... _lifetimes_ ensure that _references_ are valid as long as we need them to be."
* "... _lifetime_, which is the scope for which that _reference_ is valid."

**Therefore, the concept of lifetime applies to references**.

> For a variable, we use the term "scope".

### References and values

The `&s1` syntax lets us create a reference that **refers** to the value of `s1` (source: [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing)).

## General description

Whenever we assign _something_ (must be a reference) to a reference, we "borrow". A reference represents _a borrow_ of some owned value.

First, let's make a quick notice for non-native English speakers.

The word "borrow" can be used as a verb or as a noun (see ["borrow definition"](https://www.dictionary.com/browse/borrow)).

And, as I discovered, relatively to the Rust programming language, the word "borrow" may designate several things:

* related to an action:
  * (**a**) the action (ex: "_the function borrows the value of...").
  * (**b**) the noun (ex: "_the borrow takes place here_").
* related to _something_:
  * (**c**) **THE** (unique) reference that refers to the value of a variable (`&variable`).
  * (**d**) **A** variable whose value is **THE** reference that refers to the (borrowed) value of a variable (`let a_borrow = &variable`). _You may have more than one borrow to a given variable_.

That is, when you hear "borrow", you should interpret it relatively to the context. It is easy to identify the verbal form. However, when the noun form is used, the word may designate different things: the "_borrowed_" (c) or the "_borrower_" (d).

```rust
let value: u8 = 1;
// "&value" (a reference) is **THE** borrow that (is a reference that) refers to the value of the variable "value."
// The variable "a_borrow" is **A** borrow whose value if the reference that refers to the value of the variable "value."
let a_borrow: &u8 = &value; // The borrow occurs here.

// The function "the_borrower" borrows the value of the variable "value."
the_borrower(&value); // The borrow occurs here.
```

Using the same word in order to designate 4 separate things is not a good idea.

It is easy to differentiate (a/b) (related to the action) from (c/d) (related to the object). And it is easy to differentiate between (a) and (b). However, it may be difficult to differentiate between (c) and (d).

That's why I prefer to use these terms for (c) and (d) respectively:

* (the) "**borrowed**" (value): the _owned_ value being borrowed.
* (the) "**borrow**" (reference): **THE** (unique) reference that refers to a "borrowed" (value) <=> `&value`.
* (a) "**borrower**" (variable): **A** variable whose value is **THE** "borrow" (reference that refers to an _owner_ value).

## Properties

The lifetime of a "borrower's value" cannot outlive the "borrow lifetime" ([thanks to Quinedot](https://users.rust-lang.org/t/lifetime-and-generic-in-plain-english/87877/6)).

```rust
// Does not compile.
{
  // The scope of the variable "value" is limited to the current block.
  let value = String::new(); // the "borrowed" (value)
  // The lifetime assigned to the value of the variable "a_borrower" is "static" (see note below).
  // That said, the lifetime of the reference that refers to the "borrowed" (value) is shorter (limited to the scope of the variable "value").
  let _a_borrower: &'static String = &value; // "&value" is the "borrow"
} // local drops here
```

> As a reference lifetime `'static` indicates that the data pointed to by the reference lives for the entire lifetime of the running program.

But the lifetime of the reference that refers to a "borrowed" (value) can be greater than the scope of the "borrower" (variable) ([thanks to Quinedot](https://users.rust-lang.org/t/lifetime-and-generic-in-plain-english/87877/6)).

```rust
// "string" is a "borrower" variable. Its value is the reference that refers to a "borrowed" owned value.
fn example<'a>(string: &'a String) -> &'a String {
  // This code does compile.
  {
    // The scope of the (borrower) variable "_local" is limited to the current block.
    // That said, the lifetime of the reference that refers to the "borrowed" owned value is greater.
    let _local: &'a str = string;
  }
  return string;
}
```



