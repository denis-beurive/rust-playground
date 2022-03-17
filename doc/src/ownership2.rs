fn main() {
    let mut s: String = String::from("10");
    // Here, we borrow the value owned by s1.
    let s_ref: &mut String = &mut s;
    s_ref.push_str(" * 2");
    // It is OK to use s1 since its value has not been transferred (only borrowed).
    println!("s: {}", s);
}
