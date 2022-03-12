use super::print_current_module_path;

pub mod nesting_and_labels;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Ok, that's enough");

            break;
        }
    }
}