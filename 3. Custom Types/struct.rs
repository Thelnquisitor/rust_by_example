#![allow(dead_code)]


// Classsic c style struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Unit struct, useful for generics
struct Unit;

// Tuple struct
struct Pair(i32, f32);

// Embedding structs is allowed
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age};

    println!("{:?}", peter);

    let point: Point = Point {x: 10.3, y: 0.4};

    println!("point coordinates: ({}, {})", point.x, point.y);

    // The syntax below will set bottom_right.y to the same value as point.y. Kinda cool!
    let bottom_right = Point {x: 5.2, ..point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Unsure what this syntax means, the website states "Destructure the opint using a let binding"
    let Point {x: left_edge, y: top_edge} = point;

    let _rectangle = Rectangle {
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    
    activities();
}

fn activities() {
    // 1. Add a function rect_area which calculates the area of a Rectangle
    fn rect_area(r: Rectangle) -> f32 {
        let Point{x: tl_x, y: tl_y} = r.top_left;
        let Point{x: br_x, y: br_y} = r.bottom_right;

        let length = (tl_x - br_x).abs();
        let width = (tl_y - br_y).abs();

        return length * width;
    }

    let rectangle = Rectangle {
        top_left: Point {x: 5.0, y:5.0},
        bottom_right: Point {x: 10.0, y:0.0},
    };

    let area = rect_area(rectangle);
    println!("{}", area);


    // 2. Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner
    // on the point, and a width and height corresponding to the f32

    // I'll be honest, no idea what they mean here.
}
