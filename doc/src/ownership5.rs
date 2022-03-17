fn main() {
    let mut s1: String = String::from("10");

    // ----------------------------------------------------------------------------
    // At any given time, you can have either one mutable reference or any number
    // of immutable references.
    // ----------------------------------------------------------------------------

    let s_ref1: &mut String = &mut s1;
    (|s: &String| { println!("s: {}", s); })(s_ref1);

    // We declare an immutable reference. But remember: at any given time, you
    // can have either one mutable reference or any number of immutable references.
    // Therefore, from this point, the previously mutable reference (s_ref1) becomes
    // invalid.

    let s_ref2: &String = &s1;
    (|s: &String| { println!("s: {}", s); })(s_ref2);
    // (|s: &mut String| { println!("s: {}", s); })(s_ref1); // This would generate an error.

    // OK, let define another mutable reference. But remember: at any given time,
    // you can have either one mutable reference or any number of immutable references.
    // Therefore, from this point, the previously immutable reference (s_ref2) becomes
    // invalid. Please note that s_ref1 has been invalidated (once and for all).
    let s_ref3: &mut String = &mut s1;
    (|s: &mut String| { println!("s: {}", s); })(s_ref3);
    (|s: &mut String| { println!("s: {}", s); })(&mut s1);
    // (|s: &String| { println!("s: {}", s); })(s_ref2); // This would generate an error.
}
