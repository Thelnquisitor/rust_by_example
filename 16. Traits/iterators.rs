struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    let mut sequence = 0..3;

    // Calling the iterator explicitly!
    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    for i in 0..3 {
        println!("> {}", i);
    }

    // calls iterator 4 times, so we should see 0, 1, 1, 2
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // Skip the first 4 elements, iterate through the next four.
    //
    // So we should see 3, 5, 8, 13. This is pretty cool!
    for i in fibonacci().skip(4).take(4) {
        println!("< {}", i);
    }

    let array = [1u32, 3, 3, 7];

    println!("Iterate the following array: {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
