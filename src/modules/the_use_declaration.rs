use super::print_current_module_path;

use deeply::nested::function as other_function;

pub fn execute_example() {
    print_current_module_path(module_path!());

    other_function();

    println!("Entering block");
    {
        use deeply::nested::function;

        function();

        println!("Leaving block");
    }

    function();
}

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