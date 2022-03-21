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