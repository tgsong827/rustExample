use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    // 외부 loop를 break or continue 하는 방법.
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            //내부 루프 break;
            // break;

            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}