use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1 ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    match some_number() {
        Some(n @ 42) => println!("The Answer: {}", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }

}

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}