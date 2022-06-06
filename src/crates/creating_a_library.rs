// use super::print_current_module_path;

// pub fn execute_example() {
//     print_current_module_path(module_path!());
// }

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}