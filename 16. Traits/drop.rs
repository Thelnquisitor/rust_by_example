struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    {
        let _b = Droppable { name: "b" };

        {
            let _c = Droppable { name: "c" };

            // Interestingly enough this gets dropped first, why?
            let _d = Droppable { name: "d" };
        }
    }

    // We can manually drop too!
    drop(_a);
}
