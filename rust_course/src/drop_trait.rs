pub fn drop_trait() {
    println!("\nDrop trait! ---------------------------\n");

    // Probably not going to be used that much but is a concept
    // that should be learned
    // Drop is the equivalent of destroy in other languages

    let goblin = Creature::new("Jeff");

    // The goblin will be dropped at the end of the scope
    // which will trigger its implemented drop function
    println!("End of scope");
}

struct Creature {
    name: String,
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} has died", self.name);
    }
}
