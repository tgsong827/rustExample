use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    println!("Hello, World!");
    println!("I'm a Rustacean!");
}

pub mod comments;
pub mod formatted_print;