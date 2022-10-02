use std::ops::Add;

fn main() {

    // Shadowing
    let x = 12;
    println!("x={}", x);
    let x = "abc"; // Create a new variable.
    println!("x={}", x);

    // Mutability
    let mut y = 12;
    println!("y={}", y);
    y = 24; // Warning: must be of the same type!
    println!("y={}", y);

    // Introducing unused variables => prefix the name of the variable with "_".
    let _unused_var = 1000;

    // Integers
    let _i: u8 = 100;
    let _i: u32 = 1000;

    // Tuples
    let tuple: (u8, u16, u32) = (2^8-1, 2^16-1, 2^32-1);
    let (x, y, z) = tuple;
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("x = {}, y = {}, z = {}", tuple.0, tuple.1, tuple.2);

    // Array
    let v: [u8; 3] = [2^8-1, 2^8-2, 2^8-3];
    println!("x = {}, y = {}, z = {}", v[0], v[1], v[2]);

    // Automatic cast => no. You must explicitly perform the cast operation.
    let small: u32 = 23;
    let mut _big: u64 = small.into();

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
}
