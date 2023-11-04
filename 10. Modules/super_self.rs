fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

// We can use the keywords self and super to make it clear where the items we are trying to access are
mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        println!("called `my::indirect_call()`");

        // self refers to the current MODULE scope. So this is equivalent to calling my::function()
        self::function();
        function();

        // aka my::cool::function()
        self::cool::function();

        // this refers to the upper level scope, outside the current module
        super::function();

        {
            // this binds the function inside the cool mod to the crate scope
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
