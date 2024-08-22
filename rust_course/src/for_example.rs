pub fn for_example() {
    println!("\nfor! ---------------------------\n");

    // it's a bit unintuitive
    // this means for values of x starting in 0 and ending before 11
    for x in 0..11 {
        println!("x = {}", x);
    }

    // showing that continue/break also work
    for x in 0..11 {
        if x % 2 == 0 {
            continue;
        }
        if x >= 8 {
            break;
        }

        println!("odd x = {}", x);
    }

    // this is a bit different
    // by giving a range and calling enumerate() on it, we 
    // can have both the value in the range, as well as the
    // 'step' count, which is called index here
    // also, putting '=' when defining a range (1..=10)
    // means that range is inclusive
    // so between 30 and 41 there will be 11 steps 
    for (index, value) in (30..=41).enumerate() {
        println!("{}: {}", index, value);
    }
}
