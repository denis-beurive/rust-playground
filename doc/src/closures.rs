fn main() {
    let print = |s: String| {
        println!("s: {}", s);
    };

    let double = |x: u64| -> u64 {
        2 * x
    };

    print(String::from("ok"));
    println!("2 * x = {}", double(10));

    // "Self-executed" closure.
    println!("result is {}",
             (|x: u64| -> u64 { 2 * x })(10)
    );

    let mut s1: String = String::from("10");
    (|s1: &mut String| { println!("s: {}", s1); })(&mut s1);
    (|s2: &mut String| { println!("s: {}", s3); })(&mut s1);
    (|s3: &String| { println!("s: {}", s3); })(&s1);
}

