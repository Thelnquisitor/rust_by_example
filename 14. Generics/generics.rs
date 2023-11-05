struct A;

struct Single(A);

struct SingleGen<T>(T);

fn main() {
    let _s = Single(A);

    // Explicit generic definition
    let _char: SingleGen<char> = SingleGen('a');

    // Implicit generic definition
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}
