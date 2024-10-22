// https://doc.rust-lang.org/stable/rust-by-example/mod/use.html

use crate::deeply::nested::{my_first_function, my_second_function, AndATraitType};

// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::function as other_function;

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    my_first_function();

    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function();

        println!("Leaving block");
    }

    function();
}