fn main() {
    variable_if_let();
    enum_if_let();
}

fn variable_if_let() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter");
    } else {
        println!("I don't like letters. Let's go with an emoticon");
    }
}

fn enum_if_let() {
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // A matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // B does not match, this will not print
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // Matching on Qux(value)
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding example, if c matches 100 then it is bound to value
    if let Foo::Qux(value @ 100) = c {
        println!("c is {}", value);
    }
}
