
// Simple enum.
enum Color {
    RED,
    GREEN,
    BLUE
}

// Simple enum with types. Here we attach a value to each enum variant.
// WARNING: calls are not allowed in patterns (match v { pattern => action }).
// Thus we use string literals instead of strings.
enum NumColor {
    RED(&'static str),
    GREEN(&'static str),
    BLUE(&'static str),
    Unknown
}

enum EnumUnit { Variant1, Variant2 }
enum EnumTuple { Variant1(u8, bool), Variant2(u16, &'static str), Other }
enum EnumStruct { Variant1{a: u8, b: bool}, Other }

// Enums with associated functions and methods.
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

    // Illustrate the use of "match" to extract value for an enum variant.
    fn iso(&self) -> String {
        let data: (u16, u8, u8) = match self {
            Date::French(d, m, y) => { (*y, *m, *d) },
            Date::Us(y, m, d) =>  { (*y, *m, *d) }
        };

        return format!("{}{}{}", data.0, data.1, data.2)
    }

    fn compare(d1: Date, d2: Date) -> bool {
        let data1: (u16, u8, u8) = match d1 {
            Date::French(d, m, y) => { (*y, *m, *d) },
            Date::Us(y, m, d) =>  { (*y, *m, *d) }
        };
        let data2: (u16, u8, u8) = match d2 {
            Date::French(d, m, y) => { (*y, *m, *d) },
            Date::Us(y, m, d) =>  { (*y, *m, *d) }
        };

        return data1.0 == data2.0 &&
            data1.1 == data2.1 &&
            data1.2 == data2.2
    }
}



fn test_simple_enum() {
    let v = Color::RED;

    // Basic comparison operators cannot be used to compare enums!

    match v {
        Color::RED => { println!("RED"); },
        Color::GREEN => { println!("GREEN"); },
        Color::BLUE => { println!("BLUE"); }
    }

    if matches!(v, Color::RED) {
        println!("ok v is red");
    }
}

fn test_enum_with_variant_values() {
    let v = NumColor::RED("red");

    // WARNING: patterns must be STATICALLY defined. That is: it is not possible to use
    // function calls. That's why we used string literals for variants values.
    match v {
        NumColor::RED("red") => { println!("RED"); }
        NumColor::GREEN("green") => { println!("GREEN"); },
        NumColor::BLUE(name) => { println!("name is {}", name); },
        NumColor::Unknown => { println!("Unknown"); },
        _ => { println!("Something else"); },
    }
}

fn test_enum_with_impl() {
    let date1 = Date::French(10, 1, 2022);
    let date1 = Date::Us(2022, 10, 1);
    println!("Date: {} / {}", date1.to_str(), date1.iso());
    if Date::compare(date1, date2) {
        println!("Dates are identical")
    }

}


fn main() {
    test_simple_enum();
    test_enum_with_variant_values();
    test_enum_with_impl();
}
