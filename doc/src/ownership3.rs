fn main() {
    let s: String = String::from("10");
    // Here, we borrow the value owned by s1.
    let s_ref1: &String = &s;
    let s_ref2: &String = &s;

    // It is OK to use s1 since its value has not been transferred (only borrowed).
    println!("s: {}", s);
    println!("s_ref1: {}", s_ref1);
    println!("s_ref2: {}", s_ref2);
}
