#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    println!("\nstack and heap! ---------------------------\n");

    let p1 = origin(); // this will be in the stack
    let p2 = Box::new(origin()); // this will be in the heap

    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); // 16 bytes
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); // 8 bytes

    // an f64 variable takes 8 bytes
    // p1 takes 16 bytes due to being a struct with 2 f64 variables
    // p2 only takes 8 bytes due to it being a reference to the memory address
    // where its value is stored
    // by coincidence, an address takes 8 bytes of memory

    let p3 = *p2; // this is how you get values from an address
                  // also called unboxing
                  // values are relocated back to the stack
    println!("p3 values: ({},{})", p3.x, p3.y);
}
