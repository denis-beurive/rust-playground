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