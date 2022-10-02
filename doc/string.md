# String

```rust
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
                    result = result.add(String::from("\\").as_str());
                    set = false;
                },
                'n' => if set {
                    result = result.add(String::from("\n").as_str());
                    set = false;
                } else {
                    result = result.add( c.to_string().as_str());
                },
                'r' => if set {
                    result = result.add(String::from("\r").as_str());
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
```

