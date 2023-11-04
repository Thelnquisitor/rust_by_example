#[allow(dead_code)]

mod my_mod {

    // by default, all items in a module are private
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // use the pub modifier to override default visibility
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // similar to other languages, public can access private
    pub fn indirect_access() {
        println!("called `my_mod::indirect_access()`, that");
        private_function();
    }

    // We can nest modules too
    pub mod nested {
        pub fn function() {
            println!("called `my::mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my::mod::nested::private_function()`");
        }

        // pub(in path) means this function is only visibile(/public) within the given path.
        // In this case, this function can only be called from the my_mod module.
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("called `my_mod::nested::public_function_in_my_mod()`, that");
            public_function_in_nested();
        }

        // this essentially means its private
        pub(self) fn public_function_in_nested() {
            println!("called `called my_mod::nested::public_function_in_nested()`");
        }

        // functions declared with pub(super) are only visible within the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my::mod::nested::public_function_in_super_mod`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        nested::public_function_in_my_mod();
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    mod private_nested {
        // visibility will be restricted by the parent, so this function is only visible in my_mod, even though
        // its declared as pub(crate)
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    function();

    my_mod::function();

    my_mod::nested::function();

    my_mod::call_public_function_in_my_mod();

    my_mod::public_function_in_crate();
}
