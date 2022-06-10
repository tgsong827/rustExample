use super::print_current_module_path;
use std::fmt::Display;
use std::fmt::Debug;

pub fn execute_example() {
    print_current_module_path(module_path!());

    // Error
    // let s = S(vec![1]);
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", rectangle.area());

    // print_debug(&_triangle);
    // println!("Area: {}", _triangle.area());
}


fn printer<T: Display>(t: T) {
    println!("{}", t);
}

struct S<T: Display>(T);

//////////////////////////

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }

// #[derive(Debug)]
#[allow(dead_code)]
struct Triangle { length: f64, height: f64 }

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 { t.area() }