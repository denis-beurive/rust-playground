# Arrays

```rust
fn main() {
    // We set each element explicitly
    let v: [u8; 3] = [u8::pow(2, 7)-1, u8::pow(2, 7)-2, u8::pow(2, 7)-3];
    println!("x = {}, y = {}, z = {}", v[0], v[1], v[2]);

    // We set all elements at once (to the value 1).
    let v: [u8; 3] = [1; 3];
    println!("x = {}, y = {}, z = {}", v[0], v[1], v[2]);
}
```
