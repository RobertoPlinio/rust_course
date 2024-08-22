#![allow(dead_code)]

struct Point<T> {
    x: T,
    y: T,
}

pub fn generics() {
    println!("\ngenerics! ---------------------------\n");

    // Generics lets you reuse some data structure with different types
    // Defined by the <T> element

    let floating_point = Point { x: 2.0, y: 5.5 };
    let integer_point = Point { x: 10, y: 99 };
    let weird_point = Point {
        x: "hello",
        y: "goodbye",
    };

    println!("Integer point: ({}, {})", integer_point.x, integer_point.y);

    println!(
        "Floating point: ({}, {})",
        floating_point.x, floating_point.y
    );

    println!("String(?) point: ({}, {})", weird_point.x, weird_point.y);
}

// Generics can be of multiple types
struct CursedPoint<T, V> {
    x: T,
    y: V,
}
