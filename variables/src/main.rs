use std::ops::{Add, Index, IndexMut};
use std::collections::HashMap;

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

    // Keep in mind: you can modify a variable only when it was initialized.
    // Thus a mutable array should always be initialized.
    let mut aa: [u8; 5] = [0; 5];
    // aa[0] <=> aa.index(0)
    println!("{} / {}", aa.index(0), aa[0]);
    // aa[0] = 10:
    //   1. call the method "index_mut" that returns a mutable reference on
    //      a given element.
    //   2. then, modify the element.
    let mut v: &mut u8 = aa.index_mut(0);
    *v = 10;
    println!("{}", aa.index(0)); // -> 10
    aa[0] = 20;
    println!("{}", aa.index(0)); // -> 20


    // Passing arrays to functions.
    // https://doc.rust-lang.org/std/primitive.array.html:
    // Arrays of any size implement the Copy trait.
    // Thus arrays' values are transferred.
    let array: [u8; 10] = [1; 10];

    fn double(a: [u8; 10]) -> [u8; 10] {
        let mut result: [u8; 10] = [0; 10];
        let mut i: usize = 0;
        for x in a.iter() {
            result[i] = 2*x;
            i = i + 1;
        };
        return result;
    }
    let _r: [u8; 10] = double(array);
    let _r: [u8; 10] = double(array); // this is valid since the value has not been transferred.

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
                '\\' => if !set {
                    set = true;
                }
                else {
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
                _ => if set {
                    return Err(String::from("INvalid input!"));
                } else {
                    result = result.add(c.to_string().as_str());
                }
            };
        };
        if set {
            return Err(String::from("Invalid input!"));
        }
        return Ok(result);
    }


    // fn escape(text: String, escaper: char, chars: HashMap<&char, char>) -> String {
    //     let mut result: String = String::from("");
    //     for c in text.chars() {
    //         if c == escaper {
    //             result = result.add(escaper.to_string().add(escaper.to_string().as_str()));
    //             continue;
    //         }
    //         // if chars.contains_key(&c) {
    //         //     result = result.add(escaper.to_string().as_str() + c.to_string().as_str());
    //         //     continue;
    //         // }
    //         // result = result.add(&c.to_string().as_str());
    //     };
    //     return result;
    // }

    let intput: String = String::from("abcd\nefgh");
    let linearized: String = linearize(intput);
    println!("linearized is {}\n", linearized);
    let unserialized = unserialize(linearized);

    match unserialized {
        Ok(value) => println!("==> <{}>", value),
        Err(value) => println!("ERROR <{}>", value)
    }


}
