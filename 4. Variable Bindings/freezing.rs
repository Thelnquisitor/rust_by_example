fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;

        // We created a new variable with the same name as the one in the outer scope, which "freezes" the data.
        //
        // I don't fully understand why the data is considered "frozen". Aren't we trying to modify the immutable variable here?
        //
        // TODO: dig deeper into this, doesn't make total sense.
        // _mutable_integer = 50;
    }

    _mutable_integer = 3;
}
