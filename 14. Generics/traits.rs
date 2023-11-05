struct Empty;
struct Null;

trait DoubleDrop<T> {
    // Takes a single T param but does nothing with it
    fn double_drop(self, _: T);
}

// Implement trait for any generic param T and caller U
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // trait has been implemented for any type, so empty OR null can call it
    empty.double_drop(null);

    struct NotEmpty;
    struct MaybeEmpty;

    let not_empty = NotEmpty;
    let maybe_empty = MaybeEmpty;

    // reinforcing the above.
    maybe_empty.double_drop(not_empty);
}
