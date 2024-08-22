pub fn slices() {
    println!("\nslices! ---------------------------\n");

    // Slices are part of array operations
    // They are essentally a part of the array
    // You can lend sections of your array to be worked on a slice

    let mut array = [1, 2, 3, 4, 5];
    println!("Original array {:?}", array);

    use_slice(&mut array[1..4]); // Lending only indexes from 1 to 3
    use_slice(&mut array); // Can also lend the entire array, of course

    println!("Current state of array {:?}", array);

    // Still not really sure of when to use slices
}

fn use_slice(slice: &mut [i32]) {
    println!(
        "Received slice of {:?}. Total length: {}",
        slice,
        slice.len()
    );
    // Here I'll modify the first element of the borrowed slice
    slice[0] = 999;
    println!("Slice at 0 is now {}", slice[0]);
}
