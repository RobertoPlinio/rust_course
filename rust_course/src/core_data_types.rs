use std::mem;

pub fn example_data_types() {
    println!("\ndata types! ---------------------------\n");

    let a: u8 = 123; // u = unsigned, 8 bits, 0 -> 255
    println!("a = {}", a); // immutable

    // u = unsigned, 0 to 2^(N-1)
    // i = signed, -2^(N-1) to 2^(N-1)-1
    let mut b: i8 = 0; // -128 -> 127
    println!("b before = {}", b);
    b = 42;
    println!("b after = {}", b);

    let c = 123456789; // i32 = 32 bits = 4 bytes
    println!(
        "c = {}, takes up {} bytes, {} bits",
        c,
        mem::size_of_val(&c),
        mem::size_of_val(&c) * 8
    );

    // u8, u16, u32, u64, i8, i16, ...

    // usize isize
    let z: isize = 123;
    println!(
        "z = {}, takes up {} bytes, {}-bit system",
        z,
        mem::size_of_val(&z),
        mem::size_of_val(&z) * 8
    );

    let d: char = 'x';
    println!(
        "{} is a char, takes up {} bytes, {} bits",
        d,
        mem::size_of_val(&d),
        mem::size_of_val(&d) * 8
    );

    // f32 f54 standard IEEE754 signed!
    let e: f32 = 2.5;
    println!(
        "e = {}, takes up {} bytes, {}-bit system",
        e,
        mem::size_of_val(&e),
        mem::size_of_val(&e) * 8
    );

    let g: bool = true;
    println!(
        "boolean g = {}, takes up {} bytes, {}-bit system",
        g,
        mem::size_of_val(&g),
        mem::size_of_val(&g) * 8
    );
}