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
        Date::French(day, month,year) => {
            tts = format!("{}{}{}", year, month, day)
        },
        Date::Unknown => {
            tts = String::from("Unknown format!")
        }
    }
    println!("Date: {}", tts);
}
