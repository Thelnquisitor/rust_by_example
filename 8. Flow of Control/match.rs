fn main() {
    let number = 13;

    println!("Tell me about {}", number);

    // Rust's equivalent of a switch, can provide pattern matching functionality
    match number {
        // Single value
        1 => println!("One!"),

        // Several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),

        // Ranged INCLUSIVE (note the =) values
        13..=19 => println!("A teen"),

        // All other cases, or default. Good idea to include this, see E0004
        _ => println!("Ain't special"),
    }

    let boolean = true;

    // To elaborate a little further on the default above. It covers ALL remaining cases, which seems to be a requirement
    // when using the match keyword.
    // For example, if we were to comment out true below, we would see a compiler error (E0004).
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);

    match_bindings();
}

fn match_bindings() {
    fn age() -> u32 {
        15
    }

    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't even celebrated my first birthday yet"),

        n @ 1..=12 => println!("I'm a child of age {:?}", n),

        n @ 13..=19 => println!("I'm a teen of age {:?}", n),

        n => println!("I'm an old person of age {:?}", n),
    }

    fn some_number() -> Option<u32> {
        Some(42)
    }

    match some_number() {
        Some(n @ 42) => println!("The answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}
