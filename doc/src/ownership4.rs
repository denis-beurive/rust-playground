fn main() {
    // In this example, the lifetime of the reference is limited to the (anonymous) function that defines it.
    let mut s1: String = String::from("10");
    (|s: &mut String| { println!("s: {}", s); })(&mut s1); // Borrow as mutable (for a limited lifespan).
    (|s: &mut String| { println!("s: {}", s); })(&mut s1); // Borrow as mutable (for a limited lifespan).
    (|s: &String| { println!("s: {}", s); })(&s1); // Borrow as immutable (for a limited lifespan).
    (|s: &mut String| { println!("s: {}", s); })(&mut s1); // Borrow as mutable (for a limited lifespan).
}