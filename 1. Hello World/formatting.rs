fn main() {
    // format! returns formatted string
    // print! identical to format but prints to standard out (io::stdout)
    // println! as above but a newline is appended
    // eprint! identical to print but prints to standard err (io::stderr)
    // eprintln! as above but a new line is appended

    // Interestingly enough, can't print out a variable without using "{}"
    // let age = 53;
    // println!(age);
    
    // String formatting, {} is a catch all for any args
    println!("{} days", 31);

    // example of multiple generic {}
    println!("{} months, {} years", 5, 2);

    // Positional arguments, starting at 0
    println!("{0}, this is {1}. {1}, this is {0}", "Jerry", "Rick");

    // Named arguments, kind of cool!
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    // Rust can handle different number formatting with standard
    // format characters such as :b for binary
    println!("Base 10:                      {}",   69420);
    println!("Base 2: (binary)              {:b}", 69420);
    println!("Base 8: (octal)               {:o}", 69420);
    println!("Base 16: (hexadecimal)        {:x}", 69420);

    // Again, this is supported in format! as well!
    println!("{}", format!("Base 2: (binary)        {:b}", 69420));

    // Print with specific width
    println!("{number:>5}", number=1);

    // Pad with extra zeroes
    println!("{number:0>5}", number=1);

    // Pad on the left side!
    println!("{number:0<5}", number=1);

    // We can parameterize width with named arguments, just need to add $ at the end
    println!("{number:0>width$}", number=1, width=5);

    // Example of previously defined variables being used in formatting
    let number: f64 = 1.0;
    let width: usize = 5;

    println!("{number:>width$}");

    activities();
}


fn activities() {
    // 1. First Activity, add missing James argument
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // 2. Second Activity is uncommenting below to trigger a compiler error
    // Similar to pragma in c/c++?
    #[allow(dead_code)]
    struct Structure(i32);

    // Not all types can be formatted by default, they need to implement fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));

    // 3. Third Activity, print pi in 3 decimal places
    let pi = 3.141592;
    println!("Pi is roughly {0:.3}", pi);
}