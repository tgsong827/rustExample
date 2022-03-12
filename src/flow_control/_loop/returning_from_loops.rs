use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {}", result);
    assert_eq!(result, 20);
}