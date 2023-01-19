# Lifetimes

For every lifetime "`a`", define a type "`MyString<'a>`" that contains a reference to a "`str`" whose lifetime is "`a`".

```rust
struct MyString<'a> {
    name: String,
    value: &'a str
}
```

For every lifetime "`a`", and for every type "`T`", define a type "`MyType<'a, T>`" that contains a reference to "`T`" whose lifetime is "`a`".

```rust
struct MyType<'a, T> {
    name: String,
    value: &'a T
}
```

For every lifetime "`a`" and every lifetime "`b`", and for every type "`T`", define a type "`MyOtherType<'a, 'b, T>`" that contains a reference to "`T`" whose lifetime is "`a`", and a reference to "`T`" whose lifetime is "`b`".

```rust
struct MyOtherType<'a, 'b, T> {
    name: String,
    value: &'a T,
    other_value: &'b T
}
```

For every lifetime "`a`", define a function "`longest_by_str<'a>`" that takes 2 references to "`str`" whose lifetimes are "`a`" as parameters, and returns an optional reference to the type "`MyString`" whose lifetime is "`a`". 

```rust
fn longest_by_str<'a>(string1: &'a str, string2: &'a str) -> Option<MyString<'a>> {
    if string1.len() > string2.len() {
        return Some(MyString { name: String::from("string1"), value: string1 });
    }
    if string2.len() > string1.len() {
        return Some(MyString { name: String::from("string2"), value: string2 });
    }
    return None;
}
```


