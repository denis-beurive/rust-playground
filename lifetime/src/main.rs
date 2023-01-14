// fn lifetime1 (input: &str, output: &mut str) {
//     let r = String::from(input).add(" x 2").as_mut_str();
//     output = r;
// }

use std::ops::Add;

// It's fine: the reference stays constant.
// The referenced value, however, changes.
fn double1(input_output: &mut u8) {
    *input_output = *input_output * 2;
}

// It's fine. We don't care about the lifetime of "input".
// Indeed, we use the value of "input".
fn double2(input: &u8, output: &mut u8) {
    *output = *input * 2;
}

fn main() {
    println!("Hello, world!");
}
