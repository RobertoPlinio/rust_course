pub fn vectors() {
    println!("\nVectors! ---------------------------\n");

    // Equivalent to C# List
    // Vec<T>

    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);

    println!("Vector is {:?}", vector);

    vector.push(43);

    println!("New last element is {}", vector[vector.len() - 1]);

    // Fun fact, the id of a Vec is of type usize
    // So it'll be whatever bits my system is and is unsigned
    // Indexes cannot be negative after all, same to memory addresses
    let index: usize = 2;
    vector[index] = 999;

    println!("Vector is now {:?}", vector);

    // Vectors have a safe get to access it
    // Returns an Option
    match vector.get(10) {
        Some(x) => println!("vector[10] = {}", x),
        None => println!("Tried to access out of bounds element in vector"),
    };

    let mut other_vector = Vec::new();
    other_vector.push(10);
    other_vector.push(20);
    other_vector.push(30);

    print!("Other vector: ");
    // Iterating is also possible
    for x in &other_vector {
        print!("{},", x);
        if x == &other_vector[other_vector.len() - 1] {
            println!("");
        }
    }

    // Removing an element
    // Pop also returns an Option
    let last_elem = vector.pop();

    // Since it is an Option, a bit more work to get the value
    // Just getting last_elem will return Some(x)
    if let Some(value) = last_elem {
        println!("Last element of vector is {}", value);
    }

    // Iterating while doing pop
    while let Some(value) = vector.pop() {
        println!("Pop {}!", value);
    }

    // While on the internet I found you can use unwrap
    // if you are certain that your value is never None
    // Or use unwrap_or to default to a value in case of None
    let empty_vector = Vec::new();
    let value = empty_vector.get(0).unwrap_or(&0u8);
    println!("Empty vector value is {}", value);
}
