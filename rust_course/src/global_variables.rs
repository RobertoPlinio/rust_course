pub fn global_variables() {
    println!("\nglobal variables! ---------------------------\n");

    // all of these must be outside of a function to be global
    const MEANING_OF_LIFE: u8 = 123; // const has no fixed address
                                     // calling it will pretty much swap the value
                                     // in this case it`ll be the same as writing 123 in the print

    println!("const value is {}", MEANING_OF_LIFE);

    static STATIC_VARIABLE: i32 = 321; // static has an address and is
                                       // also not mutable

    println!("static value is {}", STATIC_VARIABLE);

    static mut MUTABLE_STATIC_VARIABLE: i32 = 000;
    // rust allows for mutable static but with one condition
    // because normally, due to various threads in the program potentially
    // reading this value at the sime time, the compiler deems
    // this operation as unsafe

    // the condition being any change done inside a unsafe block
    // this is you promising that this change is 'safe' and
    // won't compromise you're being careful
    unsafe {
        MUTABLE_STATIC_VARIABLE = 999;
        println!("mutable static value is {}", MUTABLE_STATIC_VARIABLE);
    }
}
