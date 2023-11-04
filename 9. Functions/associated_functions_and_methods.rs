struct Point {
    x: f64,
    y: f64,
}

// All Point related functions & methods go in the impl block
impl Point {
    // This is considered an associated function as it is linked to a specific type.
    //
    // Note the lack of arguments, which means we cannot refer to a specific INSTANCE of the type.
    // Often associated functions are used as constructors for the type
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // This one has arguments, but no reference to "self". Still an associated function
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // &self is syntactic sugar for self: &Self
    //
    // This is a method, we are able to access members of a specific type through the self argument
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    // &self allows read access to struct fields
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // To write fields, we need to specify mut
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // RBE describes this as a consumer method. Notice that we are not taking a reference to self here
    // so we are not borrowing the variable, we are taking ownership of it when destroy is called. Once we leave
    // this scope, first and second will be freed and the caller can no longer use the original pair variable
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

fn main() {
    let rectangle = Rectangle {
        // Examples of calling associated functions
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Square is mutable, so this is legal! This would not work on rectangle
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    // After this point, we can no longer use pair. It has been consumed.
    pair.destroy();
}
