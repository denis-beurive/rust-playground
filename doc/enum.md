# Enums

Examples: [code](src/enums.rs).

## 3 kinds of enums: unit, tuple, struct

There are 3 kinds of enum:

* Unit
* Tuple
* Struct

```rust
enum EnumUnit { Variant1, Variant2 }
enum EnumTuple { Variant1(u8, bool), Variant2(u16, &'static str), Other }
enum EnumStruct { Variant1{a: u8, b: bool}, Other }
```

> Please note that, because patterns must be STATICALLY defined, you should use "string literals"
> (`&'static str`) instead of String instances as variants values. That is: `Variant2(u16, &'static str)`.

## Get value from variant

## Unit variant

The example [here](src/enum1.rs)

```rust
enum Vehicle {
    Car(&'static str),
    Moto(&'static str),
    Other(&'static str)
}

fn main() {

    let compact_car = Vehicle::Car("Peugeot 106");

    // ---------
    let mut model: String;
    if let Vehicle::Car(name) = compact_car {
        model = String::from(name);
    }
    else {
        model = String::from("This is not a car!")
    }
    println!("Model: {}", model);

    // ---------
    match compact_car {
        Vehicle::Car(name) => { model = format!("car: {}", name) },
        Vehicle::Moto(name) => { model = format!("moto: {}", name) },
        Vehicle::Other(name) => { model = format!("unknown: {}", name) }
    }
    println!("{}", model);
}
```

## Tuple variant

The example [here](src/enum2.rs)

```rust
enum Date {
    French(u8, u8, u16), // DD, MM, YYYY
    Us(u16, u8, u8), // YYYY, MM, DD
    Unknown
}

fn main() {
    let timestamp = Date::Us(2022, 03, 21);

    // ---------
    let mut tts: String;
    if let Date::Us(year, month, day) = timestamp {
        tts = format!("{}{}{}", year, month, day);
    }
    else {
        tts = String::from("This is not a US formatted date!")
    }
    println!("Date: {}", tts);

    // ---------
    match timestamp {
        Date::Us(year, month, day) => {
            tts = format!("{}{}{}", year, month, day)
        },
        Date::French(day, month, year) => {
            tts = format!("{}{}{}", year, month, day)
        },
        Date::Unknown => {
            tts = String::from("Unknown format!")
        }
    }
    println!("Date: {}", tts);
}
```

## Struct variant

The example [here](src/enum3.rs)

```rust
enum Message {
    Hello{name: &'static str},
    Post{nature: &'static str, data: &'static str},
    Ping,
    Unknown
}

fn main() {
    let message = Message::Post {nature: "text", data: "Executing test #1"};

    // ---------
    let mut data: String;
    if let Message::Post {nature: "text", data: text} = message {
        data = String::from(text)
    }
    else {
        data = String::from("This is not a Post text message")
    }
    println!("data: {}", data);

    // ---------
    match message {
        Message::Hello {name: value} => {
            data = format!("Hello <{}>", value)
        },
        Message::Post {nature: "text", data: text} => {
            data = format!("Post text <{}>", text)
        },
        _ => {
            data = String::from("Cannot extract text data from message")
        }
    }
    println!("data: {}", data);
}
```

## Testing variants

The example [here](src/enum4.rs)

```rust
enum Message {
    Hello{name: &'static str},
    Post{nature: &'static str, data: &'static str},
    Ping,
    Unknown
}

fn main() {
    let message = Message::Post {nature: "text", data: "Executing test #1"};

    if matches!(message, Message::Post{nature: _, data: _}) {
        println!("Message is a Post");
    }
    if matches!(message, Message::Post{nature: "text", data: _}) {
        println!("Message is a text Post");
    }
    if matches!(message, Message::Post{nature: v, data: _} if v == "text") {
        // The value of "v" is not accessible here (not captured).
        println!("Message is a text Post");
    }
}
```

> Please note that :
> 
> * using `matches!`, you cannot capture the variants' values.
> * this example manipulates struct variants. But this technic also applies to Unit and Tuple variants.

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
