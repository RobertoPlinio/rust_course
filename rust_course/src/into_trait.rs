use std::fmt::Debug;

pub fn into_trait() {
    println!("\nInto trait! ---------------------------\n");

    // Into is a conversion trait, opposite of trait From
    // Consumes a value and transforms it
    // Documentation recomends using From most of the time

    // Imagening that we already have the string we want to use
    // as the name
    let my_name = String::from("Kshinuku");

    // The course says that originally
    // we might do something like this
    let p1 = Person::new_the_old_way(my_name.as_ref());
    //But uh doing this also works fine, so I'm not sure
    let p1_1 = Person::new_the_old_way(&my_name);

    // This way was supposed to fix the 'problem' above
    let p2 = Person::new(my_name);
    // my_name has been consumed and can't be used again

    // Showing the alternative works the same way
    // and passing a &str instead of the String done earlier
    let p3 = Person::new_other("Shiprur");

    println!(
        "All persons\n\t{}\n\t{}\n\t{}\n\t{}",
        p1.name, p1_1.name, p2.name, p3.name
    );

    // At least this was useful to know what Into can do
    is_hello("Hello there");
    is_hello("Yoo hoo");
    is_hello("hello");
}

struct Person {
    name: String,
}

impl Person {
    // The old way as reference
    fn new_the_old_way(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }
    // Here we are using into to allow a person to receive
    // both string types
    // So if a &str is supplied then it will convert to a String
    fn new<T>(name: T) -> Person
    where
        T: Into<String>,
    {
        Person { name: name.into() }
    }

    // Just like with traits as parameters, there is another way
    // that it can be written
    fn new_other<T: Into<String>>(name: T) -> Person {
        Person { name: name.into() }
    }
}

// Another example, where an input is compared with 'hello'
fn is_hello<T: Into<Vec<u8>> + Debug>(input: T) {
    println!("Is {:?} 'hello'?", input);
    let bytes = b"hello".to_vec();
    println!("{}", bytes == input.into());
}
