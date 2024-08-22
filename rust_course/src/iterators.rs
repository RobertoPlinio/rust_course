pub fn iterators() {
    println!("\nIterators! ---------------------------\n");

    // Ways to iterate on something

    // Assume an ordinary vector
    let vec = vec![3, 2, 1];
    println!("Old iteration:");

    // Prior, we used a for like this
    for x in vec {
        print!("{} ", x);
    }
    println!("");

    // But this is actually moving the vector value
    // Making it innacessible to be used later
    // Calling vec.iter() for example will resunt in an error
    // "borrow of moved value: 'vec' "

    let vec = vec![3, 2, 1];
    println!("Borrow iteration:");

    // So a way to do that is to borrow the vec values
    for x in &vec {
        print!("{} ", *x);
    }
    println!("");

    println!("Iter iteration:");
    // The iter creates a reference for us, so no risk of ever
    // iterating like before and missing the &
    for x in vec.iter() {
        print!("{} ", x); // Unboxing (*) here is optional, rust is smart
                          // enough to automatically put it for us
                          // in runtime
    }
    println!("");

    // Can also do it mutables
    let mut vec = vec![3, 2, 1];
    println!("Doubling values:");
    for x in vec.iter_mut() {
        *x *= 2; // Here rust is not smart enough so the * unbox is necessary
        print!("{} ", *x);
    }
    println!("");

    // Iter lets us do some extra stuff, like reversing the values
    println!("Reversing values:");
    for x in vec.iter().rev() {
        print!("{} ", *x);
    }
    println!("");

    let mut vec2 = vec![1, 2, 3];

    // Into iter transforms the vector into an iteractable
    // This is a move method, so vec will be unusable after
    let vec_into_iter = vec.into_iter();
    vec2.extend(vec_into_iter); // Appending

    println!("{:?}", vec2);
}
