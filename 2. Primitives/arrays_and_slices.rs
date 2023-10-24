use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}


fn main() {
    // Array of length 5. The website states the type signature is superfluous, but
    // if we don't specify it then what is the default?
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Initialise all 500 elements to the same value (0 in this case)
    let ys: [i32; 500] = [0; 500];  

    // Index starts at 0
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // len function returns the COUNT of elements in the array
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be "borrowed" as slices
    println!("Borrow the whole array as a slice");
    analyze_slice(&xs);

    // Take a section of the array as a slice. So far it sounds similar to GO
    println!("Borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Empty array
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // equal to the line above... so why does this syntax exist?

    // Accessing arrays, the website states using .get "safely accesses" an array.
    for i in 0..xs.len() + 1 { // out of bounds here

        // Okay this is pretty cool, saves us from shooting ourselves in the foot (but could it obscure bugs?).
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far", i),
        }

        // The .get returns an Option.
        // We can use .expect() on the option to gracefully exit the program in case of an out of bounds error.
        // xs.get(i).expect("Oh no it's out of bounds!");
    }


    // Note, since the length of a slice is not know at COMPILE time, any out of bounds error will be caught at
    // runtime.

    // Out of bound indexing on array causes compile time error.
    // println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    // println!("{}", xs[..][5]);
}
