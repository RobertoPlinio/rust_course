use std::f64;

pub fn example_operators() {
    println!("\noperators! ---------------------------\n");

    // arithmetic

    let mut a = 2 + 3 * 4; // +-*/
    println!("{}", a);
    a = a + 1; // -- and ++ doesn`t exist in rust
    a -= 2; // a = a - 2
            // -= += *= /= %=
    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, f64::consts::PI);
    println!("{} cubed is {}, {}^pi is {}", b, b_cubed, b, b_to_pi);

    //bitwise

    let c = 1 | 2; // | = OR & = AND ^ = XOR ! = NOR
                   // 01 OR 10 = 11 == 3_10

    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10; // >> also exists
    println!("2^10 = {}", two_to_10);

    //logical

    let pi_less_4 = f64::consts::PI < 4.0; // true
                                           // > <= >= ==
    println!("PI less than 4? {}", pi_less_4);
    let x = 5;
    let x_is_5 = x == 5; // true
    println!("x is 5? {}", x_is_5);
}
