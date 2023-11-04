use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    other_function();

    println!("Entering block");
    {
        // Shadowing use here
        use crate::deeply::nested::function;
        function();
        println!("Exiting block!");
    }

    function();
}
