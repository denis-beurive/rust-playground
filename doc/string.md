# String

The type "String":
* does not implement the Copy trait. Thus "=" implies a move.
* implements the Clone trait.

## How to concatenate strings

```rust
use std::ops::Add;

// --------------------------------------------------------------------------
// How to copy a string
// --------------------------------------------------------------------------

fn concat_string1(s1: &String, s2: &String) -> String {
    return (*s1).clone().add((*s2).as_str());
}

fn concat_string2(s1: &String, s2: &String) -> String {
    // Method "to_owned()" is identical to the method "clone".
    return s1.to_owned().add(s2.as_str());
}

fn concat_string3(s1: &String, s2: &String) -> String {
    // If you look at the implementation of the method "&str::to_string()", you can see that it is
    // equivalent to "String::from(self)". Thus: "concat_string4(...)".
    return s1.as_str().to_string().add(s2.as_str());
}

fn concat_string4(s1: &String, s2: &String) -> String {
    // Please read the comment for "concat_string3(...)".
    return String::from(s1.as_str()).add(s2.as_str());
}

fn concat_string5(s1: &String, s2: &String) -> String {
    return String::from(s1).add(s2.as_str());
}

// --------------------------------------------------------------------------
// Using &str instead of &String
// --------------------------------------------------------------------------

fn concat_string10(s1: &str, s2: &str) -> String {
    return s1.to_string().add(s2);
}

fn concat_string20(s1: &str, s2: &str) -> String {
    // Method "to_owned()" is identical to the method "clone".
    return s1.to_owned().add(s2);
}

fn concat_string30(s1: &str, s2: &str) -> String {
    // Please read the comment for "concat_string3(...)".
    return String::from(s1).add(s2);
}

fn main() {
    let s1 = String::from("a");
    let s2 = String::from("b");

    let s3 = concat_string1(&s1, &s2);
    println!("{} + {} = {}", s1, s2, s3);
    let s3 = concat_string2(&s1, &s2);
    println!("{} + {} = {}", s1, s2, s3);
    let s3 = concat_string3(&s1, &s2);
    println!("{} + {} = {}", s1, s2, s3);
    let s3 = concat_string4(&s1, &s2);
    println!("{} + {} = {}", s1, s2, s3);
    let s3 = concat_string5(&s1, &s2);
    println!("{} + {} = {}", s1, s2, s3);

    let s3 = concat_string10(&s1, &s2);
    println!("{} + {} = {}", s1, s2, s3);
    let s3 = concat_string20(&s1, &s2);
    println!("{} + {} = {}", s1, s2, s3);
     let s3 = concat_string30(&s1, &s2);
    println!("{} + {} = {}", s1, s2, s3);
}
```

## Looping over characters 

```rust
use std::ops::Add;

fn main() {

    // Strings
    fn linearize(text: String) -> String {
        let mut result: String = String::from("");
        for c in text.chars() {
            match c {
                '\n' => result = result.add("\\n"),
                '\r' => result = result.add("\\r"),
                '\\' => result = result.add("\\\\"),
                _ => result = result.add( c.to_string().as_str()),
            };
        };
        return result;
    }

    fn unserialize(text: String) -> Result<String, String> {
        let mut result: String = String::from("");
        let mut set: bool = false;
        for c in text.chars() {
            match c {
                '\\' => if !set { set = true; } else {
                    result = result.add("\\");
                    set = false;
                },
                'n' => if set {
                    result = result.add("\n");
                    set = false;
                } else {
                    result = result.add( c.to_string().as_str());
                },
                'r' => if set {
                    result = result.add("\r");
                    set = false;
                } else {
                    result = result.add( c.to_string().as_str());
                },
                _ => result = result.add( c.to_string().as_str()),
            };
        };
        if set {
            return Err(String::from("Invalid input!"));
        }
        return Ok(result);
    }

    let intput: String = String::from("abcd\nefgh");
    let linearized: String = linearize(intput);
    println!("linearized is {}\n", linearized);
    let unserialized = unserialize(linearized);

    match unserialized {
        Ok(value) => println!("==> <{}>", value),
        Err(value) => println!("ERROR <{}>", value)
    }
}
```
