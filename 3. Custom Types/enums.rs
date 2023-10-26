// Enums are another type we can use, but unlike c/c++ we can
// have variants of different types
enum WebEvent {
    // Unit like enums 
    PageLoad,
    PageUnload,

    // tuple enums
    KeyPress(char),
    Paste(String),

    // c style struct enum
    Click{x: i64, y: i64},
}


fn inspect(event: WebEvent) {
    // Example of how to process enums
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click {x, y} => {
            println!("clicked at x={}, y={}", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click{x:20, y:80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);   

    type_alias(); 
}

// Type aliases
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Example of type alias in impl, with Self aliasing the enum name
// impl VeryVerboseEnumOfThingsToDoWithNumbers {
//     fn run(&self, x: i32, y: i32) -> i32 {
//         match self {
//             Self::Add => x + y,
//             Self::Subtract => x - y,
//         }
//     }
// }

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn type_alias() {
    let x = Operations::Add;
}