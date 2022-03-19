# Enums

Examples: [code](src/enums.rs).

## 3 kinds of enums: unit, tuple, struct

```rust
enum EnumUnit { Variant1, Variant2 }
enum EnumTuple { Variant1(u8, bool), Variant2(u16, &'static str), Other }
enum EnumStruct { Variant1{a: u8, b: bool}, Other }
```

> Please note that, because patterns must be STATICALLY defined, you should use "string literals"
> (`&'static str`) instead of String instances as variants values. That is: `Variant2(u16, &'static str)`.

## Associated functions and methods

```rust
enum Date {
    French(u8, u8, u16), // DD, MM, YYYY
    Us(u16, u8, u8) // YYYY, MM, DD
}

impl Date {
    fn to_str(&self) -> String {
        match self {
            Date::French(d, m, y) => { return format!("{}/{}/{}", d, m, y) }
            Date::Us(y, m, d) =>  { return format!("{}/{}/{}", y, m, d) }
        }
    }
}
```
