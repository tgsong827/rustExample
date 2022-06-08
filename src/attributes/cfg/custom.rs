use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    conditional_function();
}


// 왜 안되지?
// #[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}