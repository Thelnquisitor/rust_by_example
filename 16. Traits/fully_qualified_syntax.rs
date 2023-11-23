trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "mmm".to_owned(),
        age: 28,
    };

    // as file name suggests, to distinguish between traits that contain functions with the same name,
    // we have to use this "as" syntax (known as fully qualified syntax)
    let username = <Form as UsernameWidget>::get(&form);
    let age = <Form as AgeWidget>::get(&form);

    assert_eq!("mmm".to_owned(), username);
    assert_eq!(28, age);
}
