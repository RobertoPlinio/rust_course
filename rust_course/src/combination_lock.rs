use std::io::{stdin, Write};

enum State {
    Locked,
    Failed,
    Unlocked,
}

pub fn combination_lock() {
    println!("\ncombination lock! ---------------------------\n");
    let skip = true; // turn this off to manually enter code

    // This is an example usage of the tools learned before (loop, match)
    // Some of the things might not be known to you but are needed for this
    // example unfortunately

    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    println!("Answer is 1234");

    loop {
        match state {
            State::Locked => {
                if skip {
                    println!("Automatically inserting 1234");
                    entry = "1234".to_string();
                } else {
                    print!("Enter code: ");
                    // this is to make print appear immediately
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    match stdin().read_line(&mut input) {
                        Ok(_) => {
                            entry.push_str(&input.trim_end());
                        }
                        Err(_) => continue,
                    }
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }

            State::Failed => {
                println!("FAILED");
                entry.clear(); // sets a string to an empty string
                state = State::Locked;
                continue;
            }

            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        };
    }
}
