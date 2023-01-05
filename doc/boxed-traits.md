# Slices, arrays, vectors of boxed traits

It is not possible to create arrays or vectors (that is "lists," in short) of mixed data types.
For example, you cannot create lists of elements of mixed types `u16` and `i16`.

One way to work around this _limitation_ (which is not really a limitation) is to use "boxed" values.

> Please note that a "boxed" value is a pointer to a heap allocated memory location.
> **No matter the type of a "boxed value," the pointer to the value always has the same size.**

```rust
// "DataVelocity" and "DataAcceleration" implement the trait "Data".
// Note: https://practice.rs/generics-traits/trait-object.html
//       impl <trait> for <struct> { ... }

use std::mem;

/// Trait that defines a data.
trait Data {
    fn name(&self) -> String;
}

/// Structure that defines a velocity data.
struct DataVelocity {
    object: String,
    value: u32
}

impl Data for DataVelocity {
    fn name(&self) -> String {
        self.object.clone()
    }
}

/// Structure that defines an acceleration data.
struct DataAcceleration {
    object: String,
    value: f32,
    direction: String
}

impl Data for DataAcceleration {
    fn name(&self) -> String {
        self.object.clone()
    }
}

/// Print the content of a slice of "i32".
fn print_slice_of_i32(slice: &[i32]) {
    for i in slice.iter() {
        println!("-> {}", i)
    }
}

/// Print the content of a slice of boxed data.
fn print_slice_of_boxed_data(slice: &[Box<dyn Data>]) {
    for boxed in slice.iter() {
        println!("-> {}", (*boxed).name())
    }
}

/// Print the content of a vector of boxed data.
fn print_vector_of_boxed_data(vector: Vec<Box<dyn Data>>) {
    for boxed in vector.iter() {
        println!("-> {}", (*boxed).name())
    }
}

fn main() {
    let slice = &[10, 20, 30];
    print_slice_of_i32(slice);

    let slice: &[Box<dyn Data>] = &[
        Box::new(DataVelocity { object: String::from("car"), value: 10 }),
        Box::new(DataAcceleration { object: String::from("truck"), value: 5.1, direction: String::from("N") }),
    ];
    print_slice_of_boxed_data(slice);

    let array: [Box<dyn Data>; 2] = [
        Box::new(DataVelocity { object: String::from("car"), value: 10 }),
        Box::new(DataAcceleration { object: String::from("truck"), value: 5.1, direction: String::from("N") }),
    ];
    print_slice_of_boxed_data(&array);

    let vector: Vec<Box<dyn Data>> = vec![
        Box::new(DataVelocity { object: String::from("car"), value: 10 }),
        Box::new(DataAcceleration { object: String::from("truck"), value: 5.1, direction: String::from("N") }),
    ];
    print_vector_of_boxed_data(vector);

    println!("Box<DataVelocity:      {}", mem::size_of::<Box<DataVelocity>>());     // => Box<DataVelocity:      8
    println!("Box<DataAcceleration>: {}", mem::size_of::<Box<DataAcceleration>>()); // => Box<DataAcceleration>: 8
}
```