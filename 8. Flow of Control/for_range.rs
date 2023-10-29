fn main() {
    simple_for();
    iterators();
}

fn simple_for() {
    // n will start at 1, end at 100
    // we can use 1..=101 for inclusive at the end
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn iterators() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // Iter borrows each element, so we can reuse the vec after the loop
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // Into_Iter consumes each element, so we cannot reuse the vec after the loop
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];

    // Iter_Mut borrows each element and allows us to modify the collection one element at a time
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
