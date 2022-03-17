fn main() {
    // n1 and n2 are integers (scalars).
    let n1: i32 = 10;
    let n2 = n1;
    // Because n1 and n2 values are scalars, the assignment of n1 to n2 will trigger the copy
    // of the value of n1 (that is: 10) to n2. Thus, there is no transfer of ownership of the value.
    // At the end, you get 2 values assigned to 2 variables.
    println!("n1: {}, &n1: {:p}", n1, &n1);
    println!("n2: {}, &n2: {:p}", n2, &n2); // the value of n1 is copied to n2.

    // s1 and s2 are instances of String.
    let s1: String = String::from("test");
    let s2 = s1;
    // The class String does not implement the method "Copy".
    // Thus, it is not possible to (implicitly) copy the value of s1 to s2.
    // Hence, the ownership of the value "test" is transferred from s1 to s2.
    // Conclusion: s1 does not own any more value.
    // Hence, s1 is not usable anymore.
    // At the end, you get one (and only one) value (that is: "test"). And this value is
    // owned by s2 (exclusively).
    println!("s2: {}, &s2: {}", s1, &s1); // => error (s1 does not own any value).
    println!("s1: {}, &s1: {}", s2, &s2);
}
