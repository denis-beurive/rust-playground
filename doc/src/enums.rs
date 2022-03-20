
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

// Enums with associated functions and methods.
enum Date {
    French(u8, u8, u16), // DD, MM, YYYY
    Us(u16, u8, u8), // YYYY, MM, DD
    Generic{year: u16, month: u8, day: u8, tz:&'static str}
}

impl Date {
    fn to_str(&self) -> String {
        match self {
            Date::French(d, m, y) => { return format!("{}/{}/{}", d, m, y) }
            Date::Us(y, m, d) =>  { return format!("{}/{}/{}", y, m, d) }
            Date::Generic {year:y, month:m, day:d, tz:tz} => {
                return format!("{}/{}/{} {}", y, m, d, tz)
            }
        }
    }

    // Illustrate the use of "match" to extract value for an enum variant.
    fn iso(&self) -> String {
        let data: (u16, u8, u8) = match self {
            Date::French(d, m, y) => { (*y, *m, *d) },
            Date::Us(y, m, d) =>  { (*y, *m, *d) }
            Date::Generic {year:y, month:m, day:d, tz:tz} => {
                (*y, *m, *d)
            }
        };

        return format!("{}{}{}", data.0, data.1, data.2)
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
    println!("Date: {} / {}", date1.to_str(), date1.iso());
}

fn test_matches() {
    let date1 = Date::French(10, 1, 2022);
    // You cannot get the variant values.
    if matches!(date1, Date::French(_, _, y) if y > 2000) {
        println!("This is a French date > 2000");
    }
}

fn test_if_let() {
    let date1 = Date::French(10, 1, 2022);
    // You can get the variant values.
    let year:u16;
    if let Date::French(d, m, y) = date1 {
        println!("This is a French date > 2000 {}/{}/{}", d, m, y);
        year = y;
    } else {
        year = 0;
    }
    println!("year = {}", year);
}


fn main() {
    test_simple_enum();
    test_enum_with_variant_values();
    test_enum_with_impl();
    test_matches();
    test_if_let();
}
