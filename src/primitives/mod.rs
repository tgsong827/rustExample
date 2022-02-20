use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    let mut mutable = 12; // 가변형 `i32`

    // 에러! 변수의 형은 변경될 수 없다!
    // mutable = true;

}

pub mod literal_and_operators;
pub mod tuples;
pub mod arrays_and_slices;