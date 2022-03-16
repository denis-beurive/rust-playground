fn main() {
    // Shadowing
    let x = 12;
    println!("x={}", x);
    let x = "abc"; // Create a new variable.
    println!("x={}", x);

    // Mutability
    let mut y = 12;
    println!("y={}", y);
    y = 24; // Warning: must be of the same type!
    println!("y={}", y);

    // Introducing unused variables => prefix the name of the variable with "_".
    let _unused_var = 1000;

    // Integers
    let _i: u8 = 100;
    let _i: u32 = 1000;

    // Tuples
    let tuple: (u8, u16, u32) = (2^8-1, 2^16-1, 2^32-1);
    let (x, y, z) = tuple;
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("x = {}, y = {}, z = {}", tuple.0, tuple.1, tuple.2);

    // Array
    let v: [u8; 3] = [2^8-1, 2^8-2, 2^8-3];
    println!("x = {}, y = {}, z = {}", v[0], v[1], v[2]);

    // Automatic cast => no. You must explicitly perform the cast operation.
    let small: u32 = 23;
    let mut _big: u64 = small.into();
}
