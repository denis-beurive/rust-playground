# Arrays

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
}
```
