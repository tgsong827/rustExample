use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let a_binding;

    {
        let x = 2;

        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error: 초기화 되지 않은 변수 사용
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}