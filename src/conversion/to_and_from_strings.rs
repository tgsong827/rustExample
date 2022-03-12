use std::fmt;

use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    // fmt::Display를 구현하면 자동으로 toString이 제공된다.
    // 또한 print!할 수 있게 됨.
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    println!("{}", circle);

    // Parsing a String
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}