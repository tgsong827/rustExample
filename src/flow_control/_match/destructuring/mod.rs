use super::print_current_module_path;

pub mod enums;
pub mod pointers_ref;
pub mod structs;
pub mod tuples;

pub fn execute_example() {
    print_current_module_path(module_path!());


}