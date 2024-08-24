pub fn vec_different_objects() {
    println!("\nVectors of Different Objects! ---------------------------\n");

    // How to store different types of objets in a vector

    // Normally this is an invalid operation
    let mut creatures = Vec::new();
    creatures.push(Human { name: "John-117" }); // Add a human is fine

    // But try adding a cat and it will not compile
    // Because the compiler already treats this vector as a Human
    // vector due to adding a human first
    //cretures.push(Cat::create("Pringles"));

    // One method is to use an Enum to bring the Animals together
    // Now this is a vector of Creatures
    let mut creatures = Vec::new();
    creatures.push(Creature::Human(Human { name: "John-117" }));
    creatures.push(Creature::Cat(Cat { name: "Pringles" }));

    // Of course, to get the correct values we must pattern match
    for i in creatures {
        match i {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }
    // This is cool but the pain point is needing to create a whole
    // new enum to deal with this, then having to do pattern matching
    // just to call the same talk method in all cases

    // So the other way is by transforming the values to refence
    // Meaning we box it
    // ALERT: Before I had the trait have a Create method, something
    // that returned Self. This made impossible for this Vec to work
    // It always gave an error that the trait Animal could not be
    // made into an object, I still don't know why that was the case
    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Human { name: "Jimble" }));
    animals.push(Box::new(Cat { name: "Purrfect" }));
    animals.push(Box::new(Snail { name: "Turbo" }));

    // Now it's much easier to call talk
    for a in animals.iter() {
        a.talk();
    }
}

trait Animal {
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Snail {
    name: &'static str,
}

impl Animal for Snail {
    fn name(&self) -> &'static str {
        self.name
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} is meowing!", self.name());
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello!", self.name());
    }
}

enum Creature {
    Human(Human),
    Cat(Cat),
}
