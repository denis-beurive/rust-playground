# String

The type "String" does not implement the Copy trait.

```rust
fn main() {

    // Since the type String does not implement the Copy trait, the given value
    // is moved (or transferred) from its original variable to the argument variable.
    // Thus, once passed to the function, the original variable could not be used anymore.

    fn double(s: String) -> String {
        // We clone "s". Thus "s" is not moved.
        let r: String = s.clone();
        return r.add(s.add(" x 2").as_str());
    }

    let s1: String = String::from("abcd");
    // The value of "s1" is moved (or transferred) to the argument variable.
    let s2 = double(s1);
    println!("s2 = {}", s2); // s2 = abcdabcd x 2
    
    // Since "s1" is cloned, that's the cloned value that is transferred
    // to the argument variable. Hence, "s1" can still be used.
    let s1: String = String::from("abcd");
    let s2 = double(s1.clone());
    println!("s1 = {}\ns2 = {}", s1, s2); // s2 = abcdabcd x 2
}
```

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
