use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}
#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// This syntax states the T parameter MUST implement the Debug trait
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// This means the type MUST implement the HasArea trait. This is cool!
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    empty_bounds();
    multiple_bounds();
    where_bounds();
}

// Traits dont need to include functionality, we can create bounds around empty traits
fn empty_bounds() {
    struct Cardinal;
    struct BlueJay;
    struct Turkey;

    trait Red {}
    trait Blue {}

    impl Red for Cardinal {}
    impl Blue for BlueJay {}

    // Red is empty, but this function will only take types that implement the Red Trait
    fn red<T: Red>(_: &T) -> &'static str {
        "red"
    }

    // ditto
    fn blue<T: Blue>(_: &T) -> &'static str {
        "blue"
    }

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
}

fn multiple_bounds() {
    // Multiple imports in a single statement, commented out Debug due to earlier import
    use std::fmt::Display;

    // Example of a generic function that requires the type to implement >1 traits.
    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }

    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: `{:?}`", t);
        println!("u: `{:?}`", u);
    }

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    compare_types(&array, &vec);
}

// An alternative way to specify bounds is to use the where keyword
fn where_bounds() {
    // example of a long bounded trait, can be hard to read
    // impl <A: Trait B + Trait C, D: Trait E + TraitF> MyTrait<A, D> for Type{}

    // Using where clause, which increases readability
    // impl <A,D> MyTrait<A,D> for Type where
    //     A: TraitB + TraitC,
    //     D: TraitE + TraitF {}

    trait PrintInOption {
        fn print_in_option(self);
    }

    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    let vec = vec![1, 2, 3];
    vec.print_in_option();
}
