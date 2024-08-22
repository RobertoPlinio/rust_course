#![allow(unused_assignments)]

pub fn strings() {
    println!("\nStrings! ---------------------------\n");

    // There are actually two types of strings

    // &str = String slice (utf-8)
    let s: &'static str = "Hello there!";
    // These are staticly allocated
    // Not flexible, they're static after all
    // Can be iterated on though

    for c in s.chars() {
        // chars() is an array I can manipulate
        // but it has nothing to do with the original string
        print!("{} ", c.to_uppercase());
    }
    println!("");

    // You can get a specific index by using 'nth()', it returns an Option
    if let Some(first_char) = s.chars().nth(0) {
        println!("First letter was {}", first_char);
    }

    // String = heap allocated
    // Can be grown
    // Example is creating a string with every letter of alphabet
    let mut letters = String::new();
    let mut a = 'a' as u8;

    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(", ");
        a += 1;
    }

    println!("{:?}", letters);

    // &str <> String
    let u: &str = &letters; // Possible, using & transforms a String to str

    // Concatenation
    // String + str
    let z = letters + "abc";

    // str to String
    // Both methods work the same
    let mut hello = "hello world".to_string();
    hello = String::from("Ggood bye!");

    // Insert / remove
    hello.push_str("!!!");
    hello.remove(0);

    println!("{:?}", hello);
}
