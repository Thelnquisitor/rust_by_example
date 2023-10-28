use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// Can't implement this if From has been implemented...
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

fn main() {
    // Converting a str into a string
    //
    // For clarity str is a reference string slice to a string literal or heap allocated String
    // it is immutable.
    //
    // String is a heap allocated type that can be resized and mutated as necessary.
    // let my_str = "hello";
    // let my_string = String::from(my_str);

    let num = Number::from(30);
    println!("{}", num.value);
    println!("My number is {:?}", num);

    let int = 5;
    let num: Number = int.into();
    println!("My number is: {:?}", num);
}
