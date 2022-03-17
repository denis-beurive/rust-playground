# Closure

See [this code](src/closures.rs):

```rust
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
}
```
