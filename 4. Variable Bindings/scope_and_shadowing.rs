fn main() {
    scope();
    shadowing();
}

fn scope() {
    // In main fn scope
    let long_lived_binding = 1;

    {
        // In block scope
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }

    // Error, since the variable doesn't exist in the main function scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);
}

fn shadowing() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // We still have access to shadowed_binding as seen in the previous print, but this scope takes precedence
        // so now it's a string!
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }

    println!("outside the inner block: {}", shadowed_binding);

    // same scope shadowing
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}
