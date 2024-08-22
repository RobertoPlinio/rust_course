use std::collections::HashMap;

pub fn hash_map() {
    println!("\nHash Maps! ---------------------------\n");

    // Equivalent to C# Dictionay or Js/Ts Map
    // Key/Value pairs

    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);
    shapes.insert(String::from("weird_shape"), 0);

    // Accessing
    println!("A square has {} sides.", shapes["square".into()]);
    // .into() is used to convert to a proper string
    // Though in testing the code worked fine without it

    // Can iterate based on key value pair
    for (key, value) in &shapes {
        println!("{} : {}", key, value);
    }

    // Insert can also override values if a key already exists
    // Here the .into() is necessary *shrug*
    shapes.insert("weird".into(), 150);
    println!("{:?}", shapes);

    // Get or Insert
    let circle = shapes.entry("circle".into()).or_insert(1);
    println!("A circle has {} side.", circle);

    // Entry or insert returns a reference, so to modify it you need to
    // unbox it
    *circle = 0;
    // :#? is called Pretty Print, organizes a bit more
    println!("{:#?}", shapes);

    // Not havind the .or_insert will deny compilation
    let triangle = shapes.entry("triangle".into()).or_insert(1);
    println!("Triangle still has {} sides", triangle);
}
