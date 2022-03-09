use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "samll" });

    // const 는 수정될 수 없음
    // THRESHOLD = 5;

    // LANGUAGE = "dfd";
}

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}