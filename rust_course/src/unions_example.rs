// 32 bits
union IntOrFloat {
    i: i32,
    f: f32,
}

pub fn unions_example() {
    println!("\nunions! ---------------------------\n");

    // unions are a bit weird and not really practical in rust
    // structs can do the same work while being easier to deal with
    // but some C or C++ implementations use Unions so that's why it exists

    // The key property of unions is that all fields of a union share
    // common storage. As a result, writes to one field of a union can
    // overwrite its other fields, and size of a union is determined by the
    // size of its largest field

    let an_union = IntOrFloat { i: 10 };

    // Due to its weird behaviour, its value can only be accessed in
    // unsafe blocks
    unsafe {
        println!("an_union.i = {}", an_union.i);
    }

    let int_union = IntOrFloat { i: 42 };
    let float_union = IntOrFloat { f: 55.0 };
    let test_union = IntOrFloat { i: 5 };

    match_union(int_union); // this will be ok
    match_union(float_union); // this will be ok
    match_union(test_union); // this will be weird
                             // since our matching does not try for int values,
                             // the compiler will take the bit value of our u32
                             // union an apply as a f32, which will result in a
                             // completely wrong result

    // if we try and check for an int value, it will always succeed so
    // the float values will be broken then
}

fn match_union(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("value of life");
            }
            // any IntOrFloat value
            IntOrFloat { f } => {
                println!("some float value of {}", f);
            }
        }
    }
}
