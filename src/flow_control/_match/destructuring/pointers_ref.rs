use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;

    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => {
            // *r += 1;
            println!("Got a reference to a value: {:?}", r);
        }
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_valu`: {:?}", m);
        }
    }

}