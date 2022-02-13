use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());
    // 줄 주석

    /* 블록 주석은 /* /* 중첩가능 */ */ */
}