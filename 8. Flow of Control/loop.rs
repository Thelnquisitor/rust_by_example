fn main() {
    simple_loop();
    nested_loop();
    loop_returns();
}

// Simple loop
fn simple_loop() {
    let mut count = 0u32;

    println!("Let's count until infinity");

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            // similar to other languages, skip the rest of the iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            // exits the loop
            break;
        }
    }
}

// Nested loops and using labels!
fn nested_loop() {
    #![allow(unreachable_code, unused_labels)]

    // Quite similar to GoLang, we can use labels to refer to specific loops
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // specify the outer label to break out of all loops
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

fn loop_returns() {
    let mut counter = 0;

    // We can pass values back from loops
    let result = loop {
        counter += 1;

        if counter == 10 {
            // We'll pass back 20 and store it in result
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
