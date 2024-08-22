pub fn higher_functions() {
    println!("\nHigher Order Functions! ---------------------------\n");

    // Higher order functions are functions that take and return
    // other functions
    // Works with predicaments
    // Like Array.indexOf in other languages, where a predicate is demanded
    // In rust, closures are very common to use here

    // First example is showing how to return a function, custom made
    // It will iterate in a infinite range until some limit is reached
    // Then it will sum of all even squares of numbers before reaching
    // that limit. The limit is the square itself, so if the square of
    // a number is higher than the limit, stop.
    let limit = 500;
    let mut sum = 0;
    // This is where we get a function, then store it in a variable
    let above_limit = greater_than(limit);

    // Iterating in an infinite range
    for i in 0.. {
        let square = i * i;

        // Here we call the function we got earlier
        // So this is where we are checking x < limit
        if above_limit(square) {
            break;
        } else if is_even(square) {
            sum += square;
        }
    }

    println!("Method #1: Sum of all even squares is: {}", sum);

    // This second example is doing the exact same operation
    // But using functions as arguments, using native methods
    // This example is closer to using Linq in C# for example
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, value| sum + value);
    // Whoa
    // So this ones was separated line by line for clarity
    // 1: Create a range (0..)
    // 2: Remap it into the square of the value
    // map takes a function as an argument "what should I do with the value?"
    // here we want to square it
    // So all numbers from the range are now squares of themselves
    // 3: Take each number while some condition is true
    // Here the condition is the function argument, in our case
    // it is "take while the number is less than the limit"
    // It must be a reference (&) because limit here is considered
    // also a reference. Not clear why yet
    // Then Filter gets me all values that passes some logic
    // Here I want to filter in all even values
    // Since x comes as a reference here I must unbox it (*x)
    // 4: Now merge all values by performing some logic
    // Here I want to sum all of them
    // The fold takes the initial value (0), then passes the current
    // combined value (sum) and the next value (value),
    // the function providad (sum + value)
    // tells it what to do with them which will be the new
    // combined value
    // There we have it :D

    println!("Method #2: Sum of all even squares is: {}", sum2);
}

// This is a function that returns another
// First, the original function, taking some value
// Then, it returns a function, using the impl and the argument
// which will be of type u32 here. This is called a Trait
// and it will be seen later on
// The returned function will, in itself, return a boolean
// and it is represented by the closure
fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    move |x| x > limit
}

fn is_even(x: u32) -> bool {
    x % 2 == 0
}
