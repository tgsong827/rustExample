use super::print_current_module_path;

fn foo() -> ! {
    panic!("This call never returns");
}

fn some_fn() {
    ()
}

pub fn execute_example() {
    print_current_module_path(module_path!());

    let a: () = some_fn();
    println!("This function returns and you can see this line.");

    // ....
    // 아직 진행중..
}