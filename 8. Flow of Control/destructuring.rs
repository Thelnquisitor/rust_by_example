fn main() {
    tuples();
    arrays_slices();
    enums();
    pointers_refs();
    structs();
}

fn tuples() {
    let triple = (3, -2, 4);

    println!("Tell me about {:?}", triple);

    match triple {
        // Destructure the second and third element. Unlike the other options, this FORCES our tuple to be
        // 3 elements.
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),

        // Match on first element, ignore the rest of the tuple
        (1, ..) => println!("First is `1` and the rest doesn't matter"),

        // Match on last element, ignore the previous elements
        (.., 2) => println!("Last is `2` and the rest doesn't matter"),

        // Match on first and last element, disregard all the elements in the middle
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"),

        // Default
        _ => println!("It doesn't matter what they are"),
    }
}

fn arrays_slices() {
    let array = [10, -2, 6];

    match array {
        // Destructuring the second and third element
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Ignore single value with _
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // Bind some, ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),

        // Store some in another array/slice (determined by the type being matched against)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Bind first and last, store middle in an array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}

fn enums() {
    #[allow(dead_code)]

    enum Color {
        // Specified by name
        Red,
        Blue,
        Green,

        // Tuples tied to different color models
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");

    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        // Don't need _ => since all cases are covered.
    }
}

fn pointers_refs() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;

    // Another way to specify a reference variable. What is the difference between this and the & symbol in front of the value?
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    // Reference values via ref keyword
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}

fn structs() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {} ", b, y),

        // Order is not important in matching, we can rename the variables also!
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // As with previous examples we can ignore variables too
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // But we must account for all varibes, below for example won't compile
        // Foo {y} => println!("y = {}", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };

    // Destructure faa, we can use it directly in the print statement below!
    let Foo { x: x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");
}
