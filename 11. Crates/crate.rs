// Need to link this with the library we created in the lib/ dir
//
// the command goes something like this: rustc crate.rs --extern rary=<path_to_lib>
fn main() {
    rary::public_function();
    rary::indirect_access();
}
