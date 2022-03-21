# Match and guards

The instruction `match` can be used with various kinds of variables.

```rust
struct SampleStruct {
    a: u8,
    b: u8,
    c: u8
}

fn main() {

    // Scalar
    let value: u16 = 100;
    match value {
        v if v % 2 == 0 => { println!("value is even") }
        v if v % 2 == 1 => { println!("value is odd") }
        _ => { println!("Catchall") } // mandatory, but never used.
    }
    
    // Tuple
    let v_tuple = (1, 10, 100);
    match v_tuple {
        (v1, v2, 100) if v1 < 10 && v2 + 5 < 20 => {
            println!("This is case 1");
        }
        _ => { println!("Catchall") }
    }

    // Struct
    let v_struct = SampleStruct {a: 10, b: 20, c: 30};
    match v_struct {
        SampleStruct {a:_, b: v1,c: v2} if v1 < 30 && v2 + 3 < 50 => {
            println!("This is case 1");
        }
        _ => { println!("Catchall") }
    }

    // String
    let v_string = String::from("This is the string");
    match v_string {
        v if v_string.starts_with("This") => { println!("Start with <Test>") }
        _ => { println!("Catchall") }
    }
}
```