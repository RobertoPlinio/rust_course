use std::fmt;

pub fn trait_parameters() {
    println!("\nTrait Parameters! ---------------------------\n");

    // Traits can be uses in function parameters
    // BUT NOT IN CLOSURES! BE AWARE
    // Closures infer their type at compilation time and don't change it
    // For traits to work, that won't do!

    let _triangle = Triangle {
        base: 2.0,
        height: 5.0,
    };
    println!("Triangle 2x5");
    print_info(&_triangle);

    let _square = Square { side: 1.5 };
    let bigger_square = Square { side: 10.0 };
    let _circle = Circle { radius: 2.0 };
    print_info_multiple(_square, bigger_square);

    print_info_multiple_different("triangle", _triangle, _circle);
}

// One way to pass trait as argument, this one is more readable
fn print_info(shape: &impl Shape) {
    // Here I'm doing by reference so I can reutilize the shape in
    // the example (_triangle)
    // Otherwise it will be moved
    println!("Area of shape is: {}", shape.area());
}

// This is another way, this one is better for multiple
// arguments that implement the same trait / trait combination
fn print_info_multiple<T: Shape + fmt::Debug>(shape1: T, shape2: T) {
    println!(
        "Area of shapes: {:?} = {} and {:?} = {}",
        shape1,
        shape1.area(),
        shape2,
        shape2.area()
    );
}

// The third way, this one is cool for different traits
fn print_info_multiple_different<T, P>(shape1_name: &'static str, shape1: T, shape2: P)
where
    T: Shape,
    P: Shape + fmt::Debug,
{
    println!(
        "Area of shapes:\nShape1 - {} = {}\nShape2 - {:?} = {}",
        shape1_name,
        shape1.area(),
        shape2,
        shape2.area()
    );
}

trait Shape {
    fn area(&self) -> f64;
}

struct Triangle {
    base: f64,
    height: f64,
}

// Here I'm also telling that Square will
// implement the Debug trait, but I don't have
// to manually do it, only having this tag
// is enough. Of course all of my fields must
// derive from Debug aswell. Native types are safe
#[derive(fmt::Debug)]
struct Square {
    side: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        (self.base * self.height) / 2.0
    }
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

// Manually implementing Debug, no need to be honest
// Just derive it
impl fmt::Debug for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Circle baby")
            .field("radius", &self.radius)
            .finish()
    }
}
