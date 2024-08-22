pub fn match_statement() {
    println!("\nmatch statement! ---------------------------\n");

    let country_code = 55;

    // pretty much a switch statement
    let country = match country_code {
        // it works top-down
        7 => "Russia",
        44 => "UK",
        46 => "Sweden",
        55 => "Brasil",
        1..=1000 => "unknown",
        _ => "invalid", // the default case
                        // rust will force you to cover all the possible ranges
                        // for the data type you're matching, otherwise it
                        // will result in compilation errors
    };

    println!("Country with code {} is {}", country_code, country);
}
