use std::collections::HashSet;

pub fn hash_set() {
    println!("\nHash Set! ---------------------------\n");

    // Represents a mathematical set
    // Pretty much all languages have one

    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    greeks.insert("alpha");

    println!("Hash set: {:?}", greeks);
    // Fun fact, the order of insertion don't matter
    // The print will be in any order

    greeks.insert("delta");
    // This print won't change, Hash Sets don't allow duplicates
    println!("Hash set: {:?}", greeks);

    // Insert method returns a boll
    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("vega added to hash set");
    }

    // Checking if some element exists
    if !greeks.contains("kappa") {
        println!("kappa is not present");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("delta was removed!");
    }

    println!("Hash set: {:#?}\n", greeks);

    // Sets can be compared
    // .collect transforms into a collection
    let one_to_five: HashSet<_> = (1..=5).collect();
    let six_to_ten: HashSet<_> = (6..=10).collect();
    let one_to_ten: HashSet<_> = (1..11).collect();
    let two_to_eight: HashSet<_> = (2..=8).collect();

    // Checkgin subset (every element of a set is present in another)
    println!(
        "Is {:?} a subset of {:?}?\n{}\n",
        two_to_eight,
        one_to_ten,
        two_to_eight.is_subset(&one_to_ten)
    );
    // Again the order of the numbers will be all over the place

    // Disjoint = no common elements
    println!(
        "Is {:?} a disjoint of {:?}?\n{}\n",
        one_to_five,
        six_to_ten,
        one_to_five.is_disjoint(&six_to_ten)
    );

    // Union = data from both sets
    println!(
        "Merging {:?} and {:?} results in\n{:?}\n",
        two_to_eight,
        six_to_ten,
        two_to_eight.union(&six_to_ten)
    );

    // Intersection = data present in both sets
    println!(
        "Items in {:?} and {:?} are {:?}\n",
        two_to_eight,
        six_to_ten,
        two_to_eight.intersection(&six_to_ten)
    );

    // Difference = elements of a set not present in another
    println!("Difference: {:?}\n", two_to_eight.difference(&six_to_ten));

    // Symmetric difference = union - intersection
    // Elements of both sets not present in the other
    println!(
        "Symmetric difference: {:?}\n",
        two_to_eight.symmetric_difference(&six_to_ten)
    );
}
