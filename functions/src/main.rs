
fn test_print() {
    println!("Hello, world!");
}

fn double (x: u8) -> u8 {
    return x*2;
}

fn other_double (x: u8) -> u8 {
    x*2 // No semi colon meas that this is the return value.
}


fn main() {
    test_print();
    println!("text is \"{}\"", { "this is a text" });
    println!("2 + 3 = \"{}\" ({})", { 2+3 }, 2+3);
    println!("2 + 3 = \"{}\" ({})", { let x: u8 = 2; let y: u8 = 3; x+y }, 2+3);
    println!("function double(10) = {}", double(10));
    println!("function other_double(10) = {}", other_double(10));
}
