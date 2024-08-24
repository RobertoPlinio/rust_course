#![allow(dead_code)]

use std::fmt::Debug;
use std::ops;

pub fn operator_overloading() {
    println!("\nOperator overloaing! ---------------------------\n");

    // Operators can be overloaded by using traits
    // Very useful if dealing with custom structs

    let a_simple = ComplexSimple::new(10, 20);
    let b_simple = ComplexSimple::new(5, 300);

    println!("a = {:?} b = {:?}\n", a_simple, b_simple);

    // If I try to do a + b, it will give me compiler errors
    // It will say the trait Add must be implemented
    // That's what we'll do
    let a_add = ComplexAdd::new(10, 20);
    let b_add = ComplexAdd::new(5, 300);
    println!("a + b = {:?}\n", a_add + b_add);

    // Yay it works. Unfortunatelly, I would have to do ALL OPERATIONS
    // that I want to see working on my complex number
    // The silver lining is that some operations, like equals (Eq),
    // partial equals (ParialEq), order (Ord), etc can be derived
    // But Add, AddAssign (+=), Neg (!), cannot :(
    let a_equals = ComplexEquals::new(10, 20);
    let b_equals = ComplexEquals::new(5, 300);
    println!(
        "{:?} equals {:?}? {}\n",
        a_equals,
        b_equals,
        a_equals == b_equals
    );
    println!("{0:?} equals {0:?}? {1}\n", a_equals, a_equals == a_equals);
}

// Complex number as example
#[derive(Debug)]
struct ComplexSimple<T> {
    real: T,
    imaginary: T,
}

impl<T> ComplexSimple<T> {
    fn new(re: T, im: T) -> ComplexSimple<T> {
        ComplexSimple::<T> {
            real: re,
            imaginary: im,
        }
    }
}

// Creating a separate Complex for example purposes
#[derive(Debug)]
struct ComplexAdd<T> {
    real: T,
    imaginary: T,
}

impl<T> ComplexAdd<T> {
    fn new(re: T, im: T) -> ComplexAdd<T> {
        ComplexAdd::<T> {
            real: re,
            imaginary: im,
        }
    }
}

// Here things can get a bit complicated, but let's go
// We are implementing the Add operator to our ComplexAdd struct
// Since we made ComplexAdd a generic, we are implementing Add
// to work on all generics aswell, otherwise we could just implement
// for some types, like i32, it would be simpler and look something
// like this imple ops::Add for ComplexAdd<i32> and that's it
// Here, since we are using generics, a restriction has to be put in
// so we know the generic type also implements add, and that it also
// has an output of the same generic type, that's what the
// where T: ops::Add<Output = T> is doing
// The rest is how and Add trait has to be implemented, but I could
// do whatever I wanted
impl<T> ops::Add for ComplexAdd<T>
where
    T: ops::Add<Output = T>,
{
    type Output = ComplexAdd<T>;
    fn add(self, rhs: Self) -> Self::Output {
        ComplexAdd {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ComplexEquals<T> {
    real: T,
    imaginary: T,
}

impl<T> ComplexEquals<T> {
    fn new(re: T, im: T) -> ComplexEquals<T> {
        ComplexEquals::<T> {
            real: re,
            imaginary: im,
        }
    }
}
