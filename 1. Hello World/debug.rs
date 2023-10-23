fn main() {
    // We can derive the fmt::Debug trait to add a default printing implementation to non std types
    #[derive(Debug)]
    struct DebugPrintable(i32);

    // We use the char :? to print Debug types, this can be used with all std types 
    println!("Int = {:?}", 25);
    println!("DebugPrintable = {:?}", DebugPrintable(52));        


    // We can use pretty printing instead, but if we don't specify a custom implementation of Debug then we can't control the way
    // types are printed. The line below will print:
    /*
        DebugPrintable(
            52,
        )
    */
    println!("DebugPrintable pretty = {:#?}", DebugPrintable(52));

    // Nested struct example
    #[derive(Debug)]
    struct DeepDebug(DebugPrintable);

    println!("DeepDebug = {:?}", DeepDebug(DebugPrintable(52)));
    println!("DeepDebug pretty = {:#?}", DeepDebug(DebugPrintable(52)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 27;
    let person = Person {name, age};

    // Just to avoid dead code warnings.
    println!("{}", person.name);
    println!("{}", person.age);

    println!("{:?}", person);
    println!("{:#?}", person);
}
