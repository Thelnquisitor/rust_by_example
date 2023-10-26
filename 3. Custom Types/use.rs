#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// We can specify the use keyword to bring enum variants into scope without
// the need to specify them via their Prefix, in this case Status or Work.
//
// Seems handy, but may code legibility may suffer.
fn main() {
    use crate::Status::{Poor, Rich};

    use crate::Work::*;

    let status = Poor;

    let work = Civilian;

    match status {
        Rich => println!("Rich boi"),
        Poor => println!("Poor boi"),
    }

    match work {
        Civilian => println!("Move along sir"),
        Soldier => println!("Thank you for your service"),
    }
}