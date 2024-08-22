#![allow(unused_parens)]

pub fn tuples() {
    println!("\ntuples! ---------------------------\n");

    // Basically several values taken together without a defined structure

    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("x = {}/ y = {}", x, y);
    println!("Sum and product is: {:?}", sp);

    // Two things here:
    // 1. You can use indexes to better organize your prints
    // *  {0} will take the first value, {1} the second, etc
    // 2. Values of tuples also have indexes, so the first
    // *  tuple value is tuple.0, and so on
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // Using indexes to access tuples can be confusing, so we can
    // do something called destructuring, which lets us label
    // what the values of our tuple are
    let sp2 = sum_and_product(10, 20);
    let (sum, product) = sp2;
    println!("x = 10 / y = 20\nSum is {}\nProduct is {}", sum, product);

    // Tuples of tuples are also possible
    // Cursed
    let combined = (sp, sp2);
    println!("Combined tuples are {:?}", combined);

    // They can also be deconstructed
    // But should you do it?
    let ((sum1, prod1), (sum2, prod2)) = combined;

    // Tuples of a single element are also possible, but... c'mon
    // In the backend this will be a simple variable
    let single_tuple = (42);

    // To be a real tuple, you can just add an empty space
    let single_tuple_ghost = (42,);
}

// Defining a tupple is a matter of putting the types in rounded brackets
// (u8, u8)
// (f32, String)
fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}
