// There are basic implementations for a variety of traits. We can
// specify these via the derive attribute. Here are some:
//
// Eq, Partial Eq, Ord, Partial Ord for comparison
// Clone to create a new T from a &T
// Copy to give a type copy semantics instead of move semantics
// Hash to compute a hash from a type
// Default to create an empty instance of a data type
// Debug to format a value using the {:?} formatter

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

fn main() {
    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter", cmp);
}
