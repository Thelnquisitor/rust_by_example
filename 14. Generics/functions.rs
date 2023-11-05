struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {}

// This function uses a generic parameter but is specified as an i32, so the function is not generic
fn gen_spec_i32(_s: SGen<i32>) {}

// generic function
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Non generic functions
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    // Explicit type specified when calling generic function
    generic::<char>(SGen('a'));

    // Implicit type specified when calling generic function
    generic(SGen('c'));
}
