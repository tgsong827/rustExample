use super::print_current_module_path;

pub mod destructuring;
pub mod binding;
pub mod guards;

pub fn execute_example() {
    print_current_module_path(module_path!());
}