fn main() {
    // Tuples may contain several different types
    let long_tuple = (1u8, 2u16, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    println!("Tuple first value: {}", long_tuple.0);
    println!("Tuple second value: {}", long_tuple.1);

    // This includes tuples inside of tuples!
    let tuples_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
 
    println!("Nested tuples: {:?}", tuples_of_tuples);

    // Apparently there is a size limit, if a tuple has more than 12 elements then it cannot be printed
    // let too_long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    // println!("This won't print! {:?}", too_long_tuple);

    // struct longTuple(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32);
    


    // example of a pair tuple
    let pair = (1, true);
    println!("Pair: {:?}", pair);

    // We can create one element tuples, which is denoted by a comma to distinguish them from a literal value 
    // inside parenthesis
    println!("One element tuple: {:?}", (5u32,));
    println!("Single integer: {:?}", (5u32));

    // Destructuring tuples to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    activities();
}


fn activities() {
    // 1. Add the display trait to the matrix struct and output matrix like this:
    // (1.1 1.2)
    // (2.1 2.2)
    use std::fmt;
    
    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({} {})\n", self.0, self.1)?;
            write!(f, "({} {})", self.2, self.3)
        }
    }
    
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);

    // 2. Add a transpose function, which accepts a matrix as an argument     
    // and returns a matrix in which two elements have been swapped
    fn transpose(m: Matrix) -> Matrix {
        return Matrix(m.0, m.2, m.1, m.3);
    }

    let transposed_matrix = transpose(matrix);
    println!("{:?}", transposed_matrix);
    println!("{}", transposed_matrix);
}