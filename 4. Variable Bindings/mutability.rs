fn main() {
    let _immutable_binding = 1;
    // _immutable_binding += 1; big no

    // Variables are immutable by default, we can override this using the mut modifier
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);
}
