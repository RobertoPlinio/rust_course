struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

// Declaring a method
impl Line {
    // &self is mandatory
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;

        // Returning a value does not need a keyword
        // and neither a semi colon ';'
        (dx * dx + dy * dy).sqrt()
    }
}

pub fn methods() {
    println!("\nMethods! ---------------------------\n");

    // They work a little bit differently than functions
    // Methods are functions in structs
    // A method is declared above

    let p1 = Point { x: 2.0, y: 3.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let my_line = Line { start: p1, end: p2 };

    println!("My line length: {}", my_line.len());
}
