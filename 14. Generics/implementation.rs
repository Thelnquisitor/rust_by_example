struct Val {
    val: f64,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

struct GenVal<T> {
    gen_val: T,
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

// Looks like you can define different impl blocks for generic types.
//
// It doesn't seem to be able to tell apart functions within the different impl scopes,
// if I have the blow function named as value then it throws a compilation error.
impl GenVal<i64> {
    fn valuei64(&self) -> &i64 {
        println!("owwowowowowo");
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());

    let z = GenVal { gen_val: 3i64 };

    println!("{}", z.valuei64());
}
