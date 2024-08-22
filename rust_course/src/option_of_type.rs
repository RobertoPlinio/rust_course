pub fn option_of_type() {
    println!("\noption<T>! ---------------------------\n");

    // Option<T> is a cool concept when you need something that
    // returns either some value or nothing

    try_division(3.0, 2.0);
    try_division(15.0, 0.0);
}

fn try_division(x: f32, y: f32) {
    println!("Trying division {}/{}", x, y);

    let result = if y != 0.0 {
        Some(x / y) // Some(value) is the whatever value we want
    } else {
        None // In case of failure, we return None
    };

    // Option<T> can be used in matching expressions
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Error: Cannot divide by zero"),
    }

    // It can also be used in if statements
    // It will try to assign result to Some(value)
    // And this will be either true or false
    if let Some(value) = result {
        println!("Success!");
    }

    // This logic can also apply to while statements
    /*
      while let Some(value) = result {
        // Do Stuff
      }
    */
}
