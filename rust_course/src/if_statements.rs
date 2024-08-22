#![allow(unused_parens)]

pub fn if_statements() {
    println!("\nif statements! ---------------------------\n");

    let temp = 15;

    // using { } is mandatory

    if temp > 30 {
        println!("Really hot outside");
    } else if (temp < 10) {
        // using ( ) is optional
        println!("Really cold outside");
    } else {
        println!("Outside is ok");
    }

    // they are an expression, so can be used like so
    let weather = if temp > 20 { "sunny" } else { "cloudy" };
    println!("Weather is {}", weather);

    let hum = 90;
    // another way
    println!(
        "Humidity is {}",
        if hum > 85 {
            "good"
        } else if hum > 50 {
            ("average")
        } else {
            "bad"
        }
    );
}
