trait Animal {
    // Self refers to the implementor type (in this example sheep!)
    fn new(name: &'static str) -> Self;

    // Just the function signatures, behaviours need to be defined when implemented
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // We can have default logic in traits, can be overriden when implemented
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }

    // IF a trait method does not have a default implementation then it MUST be defined when implemented.
    // Uncomment to see this in action
    // fn implement_me(&self) -> &'static str;
}

struct Sheep {
    naked: bool,
    name: &'static str,
}

impl Sheep {
    fn is_naked(&self) -> bool {
        // Still feels weird without the return keyword
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    // As mentioned above, this returns the implementor type
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }

    // Defining the functions above
    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baahahahahah"
        } else {
            "baaaaaaaaaaaaaaaaaaaaaaaaaaa"
        }
    }

    // Overriding the default method implementation in the trait
    fn talk(&self) {
        println!("{} pauses briefly {}", self.name, self.noise());
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
}
