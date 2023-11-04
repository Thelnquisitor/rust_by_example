fn main() {
    closures();
    capturing();
    closures_as_params();
    input_functions();
    output_parameters();
}

// Closures are anonymous functions that can access its surrounding scope.
//
// Similar to anonymous functions or lambda expressions in other languages.
fn closures() {
    let outer_var = 42;

    // Explicit representation of a closure
    let closure_annotated = |i: i32| -> i32 { i + outer_var };

    // Infered, we don't need to specify the type, the return or even the body braces
    let closure_inferred = |i| i + outer_var;

    println!("annotated: {}", closure_annotated(1));
    println!("inferred: {}", closure_inferred(1));

    // Example of a closure that takes no arguments
    let one = || 1;

    println!("closure one: {}", one());
}

// We can capture variables in closures by borrowing /
fn capturing() {
    use std::mem;

    let color = String::from("green");

    // This will borrow color and will only be released after print is called the last time
    let print = || println!("`color`: {}", color);

    print();

    // We can reborrow immutably according to RBE
    let _reborrow = &color;

    print();

    // We can only take ownership of the variable after the final use of `print`
    let _color_moved = color;

    let mut count = 0;

    // Borrows count, is mutable
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // call the closure with a mutable borrow. We cannot reborrow since inc is called later
    inc();
    inc();

    // The closure no longer needs to borrow &mut count allowing us to reborrow here
    let _count_reborrowed = &mut count;

    // Heap allocated type
    let movable = Box::new(3);

    // movable is moved into the closure, and then is consumed by the call to mem::drop
    // so we can only call consume once and can no longer use movable after it has been called.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();

    // We can force the closure to take ownership by using move before the pipes
    let haystack = vec![1, 2, 3];

    // haystack has been moved into the closure, we can no longer reference it outside.
    //
    // if we remove the move keyword, the closure will borrow haystack immutably which means we can
    // reference it beyond the scope of the closure.
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
}

fn closures_as_params() {
    // When taking closures as arguments, we need to annotate them with one of three traits:
    // Fn - closure uses value by reference (not mutable)
    // FnMut - closure uses value by mutable reference
    // FnOnce - closure uses values by value
    //
    // Also note the syntax, F is a generic type
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzz");

        mem::drop(farewell);
    };

    // Diary takes no arguments, will be used as value
    apply(diary);

    let double = |x| 2 * x;

    // double will be used by reference
    println!("3 doubled: {}", apply_to_3(double));
}

// Showing that functions can be used in place of closures as long as they satisfy
// the bound trait in the called function.
fn input_functions() {
    // Similar syntax to above, although slightly easier to look at IMHO
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    fn function() {
        println!("I'm a function!");
    }

    let closure = || println!("I'm a closure");

    call_me(closure);
    call_me(function);
}

fn output_parameters() {
    // We can return closures from functions, since closures are anonymous types,
    // we need to use the impl Trait syntax.
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();

        // Using move will cause all captures to occur by value. Essentially the closure
        // takes ownership of the variables it captures
        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let mut text = "FnMut".to_owned();

        move || {
            text.push_str("Hello!");
            println!("This is a: {}", text)
        }
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();

        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
