# tuple

```rust
fn main() {
    let tuple: (u8, u16, u32) = (2^8-1, 2^16-1, 2^32-1);
    let (x, y, z) = tuple;
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("x = {}, y = {}, z = {}", tuple.0, tuple.1, tuple.2);
}
```
