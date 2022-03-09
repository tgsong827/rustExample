use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    // _immutable_binding += 1;
}