pub fn scope_and_shadowing() {
    println!("\nscope and shadowing! ---------------------------\n");

    let a = 123;
    let a = 1234; // rust lets you declare a variable again,
                  //but will consider the older variable as dead code

    //creating a scope
    {
        println!("Entering a scope");

        let b = 567; // b will be locked in this scope
        println!("***value of b is {}", b);

        let a = 777; // this a is exclusive to this scope, and will
                     // overshadow the previously created a, this is dangerous
                     // if not for this new a, calling the value of a would return its
                     // original value
        println!("***value of shadowing a is {}", a);

        println!("Leaving scope");
    }

    println!("value of a is {}", a); // since we left the earlier scope,
                                     // the value of a will be the one defined earlier

    //println!("value of b is {}", b); // this can`t be done since b is outside
    //of this scope`
}
