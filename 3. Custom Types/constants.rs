// Similar to other languages, convention seems to be consts are written in ALL CAPS

// A mutable constant?
static LANGUAGE: &str = "Rust";

// Standard unchangeable value.
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});
}
