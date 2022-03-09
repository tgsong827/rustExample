use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    use Status::{Poor, Rich};
    use Work::*;

    // Status::Poor 과 같음.
    let status = Poor;
    // Work::Civilian 과 같음.
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}