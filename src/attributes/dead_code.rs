use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    used_function();
}

fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}