pub fn traits() {
    println!("\nTraits! ---------------------------\n");

    // This one is a mish mash of different things I'm used to in OOP
    // Traits are inheritance, interfaces and extension methods
    // at the same time
    // Ths most common explanations of traits is? they define
    // shared behaviour

    let _snail = Snail { name: "Turbo" };
    _snail.talk();

    let _cat = Cat {
        name: "Marechal RonRon",
    };
    _cat.talk();

    // Can also call directly the original trait
    let _human: Human = SocialAnimal::create("Antedeguemon");
    _human.talk();

    // Using a custom trait to sum a vec of i32
    let _vec = vec![1, 2, 3, 4];
    println!("Sum of {:?} is {}", _vec, _vec.sum());
}

// Defining a trait, this is an animal that has a name
// and may or may not speak
trait Animal {
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

// Simple implementation
struct Snail {
    name: &'static str,
}

// Here we are connecting Snail and Animal together
impl Animal for Snail {
    fn name(&self) -> &'static str {
        self.name
    }
}

struct Cat {
    name: &'static str,
}

// For cat we are overriding the talk method
// to do something different
impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} is meowing!", self.name());
    }
}

// Creating another Animal trait so the other animals are
// not changed
// Pretend this is the same Animal as seen above
trait SocialAnimal {
    // We can have a method that acts as a constructor
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self);
}

struct Human {
    name: &'static str,
}

impl SocialAnimal for Human {
    // The 'constructor' must be implemented by any who uses
    // the trait
    fn create(name: &'static str) -> Human {
        println!("A baby is born!");
        return Human { name: name };
    }
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello!", self.name());
    }
}

// Creating summable to work like an extension method
trait Summable<T> {
    fn sum(&self) -> T;
}

// Implementing my 'extension method' to work on a vec of i32
impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result = 0;
        for i in self {
            result += *i;
        }

        return result;
    }
}
