pub fn while_and_loop() {
    println!("\nwhile and loop! ---------------------------\n");

    let mut x = 1;

    while x < 1000 {
        x *= 4;

        if x > 1000 {
            continue; // skip to next iteration
        }

        println!("x = {}", x);
    }

    let mut y = 1;
    // loop is equivalent to while = true
    loop {
        y *= 4;

        // 1 << 10 is the same as 2^10, which is 1024
        if y >= 1 << 10 {
            break; // break and exits the loop
        }

        println!("y = {}", y);
    }
}
