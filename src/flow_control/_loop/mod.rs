use super::print_current_module_path;

pub mod nesting_and_labels;

pub fn execute_example() {
    print_current_module_path(module_path!());


}