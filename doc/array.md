# Arrays

Arrays of any size implement the Copy trait.

> See [the list](https://doc.rust-lang.org/std/marker/trait.Copy.html#implementors) of all types that implements 
> the Copy trait. Please note that the type "array" is listed as "`impl<T, const N: usize> Copy for [T; N]`".

Thus: array values are not transferred (from a variable to another one).

```rust
fn main() {
    // We set each element explicitly
    let v: [u8; 3] = [u8::pow(2, 7)-1, u8::pow(2, 7)-2, u8::pow(2, 7)-3];
    println!("x = {}, y = {}, z = {}", v[0], v[1], v[2]);

    // We set all elements at once (to the value 1).
    let v: [u8; 3] = [1; 3];
    println!("x = {}, y = {}, z = {}", v[0], v[1], v[2]);
    
    // Keep in mind: you can modify a variable only when it was initialized.
    // Thus a mutable array should always be initialized.
    let mut aa: [u8; 5] = [0; 5];
    // aa[0] <=> aa.index(0)
    println!("{} / {}", aa.index(0), aa[10]);
    // aa[0] = 10:
    //   1. call the method "index_mut" that returns a mutable reference on
    //      a given element.
    //   2. then, modify the element.
    let mut v: &mut u8 = aa.index_mut(0);
    *v = 10;
    println!("{}", aa.index(0)); // -> 10
    aa[0] = 20;
    println!("{}", aa.index(0)); // -> 20
    
    // Passing arrays to functions.
    // https://doc.rust-lang.org/std/primitive.array.html:
    // Arrays of any size implement the Copy trait.
    // Thus arrays' values are _NOT_ transferred.
    let array: [u8; 10] = [1; 10];

    fn double(a: [u8; 10]) -> [u8; 10] {
        let mut result: [u8; 10] = [0; 10];
        let mut i: usize = 0;
        for x in a.iter() {
            result[i] = 2*x;
            i = i + 1;
        };
        return result;
    }
    
    let _r: [u8; 10] = double(array);
    let _r: [u8; 10] = double(array); // this is valid since the value has not been transferred.
}
```
