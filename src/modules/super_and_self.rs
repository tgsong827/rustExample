use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    my::indirect_call();
}

fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

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
        print!("called `my::indirect_call()`, that\n> ");

        self::function();
        function();

        self::cool::function();

        super::function();
        {
            use crate::modules::super_and_self::cool::function as root_function;
            root_function();
        }
    }
}