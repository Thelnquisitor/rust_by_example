use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    // Convert struct to string
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // Convert string to a number

    // type inference
    // Must be careful as calling unwrap can panic if the underlying Result did not return as OK
    let parsed: i32 = "5".parse().unwrap();

    // "turbofish" syntax, explicit type
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
