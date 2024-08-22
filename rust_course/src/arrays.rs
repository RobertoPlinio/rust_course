use std::mem;

pub fn arrays() {
    println!("\narrays! ---------------------------\n");

    // declaring an array of 5 items of type 32-bit integer
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Array a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;
    println!("First value of a is now {}", a[0]);

    assert_eq!(a, [321, 2, 3, 4, 5]);

    // cannot compare arrays of different sizes!!!
    if a != [1, 2, 3, 4, 5] {
        // :? is a debug option for printing
        println!(
            "Array a is not equal to its default values. [1, 2, 3, 5, 6] is now {:?}\n",
            a
        );
    }

    // fill an array with ones
    let b = [1u16; 10]; //1u16 is me forcing the 1 to be of type
                        // u15, otherwise defaults to u32 which takes up more memory
    println!("Array b values: ");
    for i in 0..b.len() {
        print!("{} ", b[i]);
        if i == b.len() - 1 {
            println!("");
        }
    }

    println!("b took up {} bytes\n", mem::size_of_val(&b));

    // multidimensional array
    // My matrix is an array of 3 elements, each being another array
    // of 3 32-bit floats
    let mtx: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [1.0, 2.0, 3.0]];
    println!("Matrix is: {:?}", mtx);

    // print all the diagonal values
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}
