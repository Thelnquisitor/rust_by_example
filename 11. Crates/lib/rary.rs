// Example of building libraries in rust
//
// rustc by default builds a binary, need to override it with the flag --crate-type=lib

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    println!("called rary's `indirect_access()`");
    private_function();
}
