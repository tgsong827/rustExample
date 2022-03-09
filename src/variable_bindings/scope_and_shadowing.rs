use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    // main 함수 내에 유효
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }

    // Error
    // println!("outer short: {}", short_lived_binding);

    // inner에서 shadowing한 값이 아닌 맨위에 바인딩한 값이 출력됨.
    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}