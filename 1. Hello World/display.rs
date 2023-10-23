use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

// We can implement fmt::Display instead of deriving from fmt::Debug to print a non std type.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0,14);

    println!("Compare Structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
    
    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big rage is {big} and the small is {small}",
    small = small_range,
    big = big_range);

    let point = Point2D {x: 3.3, y: 7.2};

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    

    activity();
    list_activity()
}

fn activity() {
    // Add a struct with the name Complex
    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,        
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}", self.real, self.imag)
        }
    }

    let complex = Complex { real: 3.3, imag: 7.2 };

    println!("Compare Complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

fn list_activity() {
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;

            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                if count != 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", count, v)?;
            }

            write!(f, "]")
        }
    }


    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}