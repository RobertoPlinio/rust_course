#![allow(unused_must_use)]

pub fn closures() {
    println!("\nClosures! ---------------------------\n");

    // They`re weird, but act as an alternative to functions on the fly
    let some_variable = 42;

    // This works but I can't access values outside of its scope
    // They call it 'capture'
    fn le_function() {
        println!("I am an annonimous function :D");
        //println!("I am an annonimous function :D {}", some_variable);
        // Uncomment line above to see a compiler error
    }

    le_function();

    // To have a function-like behaviour while capturing outside values
    // We use closures
    || {}; // Yes, this is an empty closure, don't do this at home
           // Here the || substitures () seen in functions

    // Closures can be added in variables
    // Functions too
    let function_as_variable = le_function;

    let closure = || println!("I am a closure!");
    // Closures can be one liners if they have only one instruction

    // Calling a closure
    closure();

    // They can take parameters and return values
    let add_one = |x: i32| -> i32 { x + 1 };

    println!("1 + 1 = {}", add_one(1));

    // Can also infer types, but will give erros if no calls are made to it
    let add_two = |x| x + 2;
    println!("1 + 2 = {}", add_two(1));

    let return_one = || 1; // Inferring return type
    println!("{}", return_one());

    // Closure greatest value, capturing variables
    let using_outer_values = |x| {
        let result = x + some_variable;
        (result, result % 2 == 0)
    };

    let (result, odd_even) = using_outer_values(42);
    println!("{} is {}", result, if odd_even { "even" } else { "odd" });
}
