#![allow(unused_variables)]
#![allow(dead_code)]

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

pub fn structures() {
    println!("\nstructures! ---------------------------\n");

    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };

    println!(
        "My line starts at ({},{}) and ends at ({},{})",
        p1.x, p1.y, p2.x, p2.y
    );

    let my_line = Line { start: p1, end: p2 };
}
