// An example of type inference, we specify an empty vector with no type and it infers
// the type when we push an element of type u8 into it.
fn main() {
    let elem = 5u8;

    let mut vec = Vec::new();

    // Rust will be able to determine what type vec is when we push elem.
    // If we comment this out, then the compiler will throw an error as it needs to know the type of vec
    vec.push(elem);

    println!("{:?}", vec);
}
