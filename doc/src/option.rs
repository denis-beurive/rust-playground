fn main() {

    let a: u16 = 10;
    let b: Option<u16>;

    // Depending on a condition, a variable may be defined or not.
    if a < 20 {
        b = Some(100)
    } else {
        b = None
    }

    // if ... then ... else
    if b.is_some() {
        println!("b is defined. Let's calculate: {}", a + b.unwrap())
    } else {
        println!("b is not defined!")
    }

    // match
    match b {
        b if b.is_some() => { println!("b is defined. Let's calculate: {}", a + b.unwrap()) }
        _ => { println!("b is not defined!") }
    }

    match b {
        Some(v) if v == 100 => { println!("b is defined and its value is 100. Let's calculate: {}", a + v) },
        Some(v) if v < 100 => { println!("b is defined and its value is lower than 100. Let's calculate: {}", a + 2*v) },
        _ => { println!("b is not defined!") }
    }
}