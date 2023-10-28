fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // If we don't use this, then the compiler will complain about dead code.
    // To get around this warning, we can prefix the variable name with an underscore
    let _unused_variable = 3u32;
}
