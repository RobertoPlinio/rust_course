pub fn dynamic_static_dispatch() {
    println!("\nDynamic and Static Dispatch! ---------------------------\n");

    // Dispatching is basically how function calls are directed
    // to the correct function definition basically
    // When you call a function, what function will the code
    // actually run
    // Dynamic and Static Dispatching are two ways that we can
    // write functions taking advantage of this behaviour

    let a = 123;
    let b = "Hello I want to talk with ANTEDEGUEMON".to_string();

    // Previously we could do something of the type
    // println!("{}", a.format());
    // println!("{}", b.format());
    // In this example we'll create a generic function to do the printing

    // The static way
    // More efficient, dispatches at compile time
    // Requires more setting up
    // And can't be always used, example later
    print_static(a);
    print_static(b);

    let c = 999;
    let d = "There's no such person with that name".to_string();

    // The dynamic way
    // Less efficient, dispatches at runtime
    // Has cases where it's the only solution
    print_dynamic(&c);
    print_dynamic(&d);
    println!("\n");

    // One such cases is this one
    // Suppose we have a vector of 5 different shapes
    // The compiler made me add the dyn keyword here
    // because just a Shape by itself has no real value,
    // but boxing it (tranforming into an address) has a known
    // value.
    let shapes: [&dyn Shape; 5] = [
        &Square { side: 1.0 },
        &Circle { radius: 1.0 },
        &Square { side: 10.0 },
        &Circle { radius: 15.0 },
        &Square { side: 3.33 },
    ];

    // Now I want to calculate the area for all of them
    // Calling shape.area() is doing a dynamic dispatch :)
    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape {}: {}", i, shape.area());
    }
}

trait Printable {
    fn format(&self) -> String;
}

// The compiler made me add the dyn keyword here
fn print_dynamic(input: &dyn Printable) {
    println!("{}", input.format());
}

fn print_static<T: Printable>(input: T) {
    println!("{}", input.format());
} // monomorphisation

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i64: {}", *self)
    }
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}
