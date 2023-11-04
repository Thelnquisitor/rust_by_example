#[allow(dead_code)]

mod my {
    // Public generic struct with a public field
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // Public generic struct with a private field
    pub struct ClosedBox<T> {
        contents: T,
    }

    // Private struct with private field, can't be accessed outside the mod
    // struct PrivateBox<T> {
    //     contents: T,
    // }

    impl<T> ClosedBox<T> {
        // constructor (public ofc)
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

fn main() {
    let open_box = my::OpenBox {
        contents: "public information",
    };

    println!("The open box contains: {}", open_box.contents);

    // We can use the constructor but won't be able to access the contents field directly
    let _closed_box = my::ClosedBox::new("classified information");
}
