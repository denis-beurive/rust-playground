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
