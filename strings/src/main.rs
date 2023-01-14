use std::ops::Add;

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