// RBE says this form is seldom used, I very much doubt that there aren't use cases where
// you'd like to declare a variable without initializing it
fn main() {
    // Can declare without initializing.
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Not initialized, will throw an error
    // println!("another binding: {}", another_binding);

    another_binding = 1;
    println!("another binding: {}", another_binding);
}
