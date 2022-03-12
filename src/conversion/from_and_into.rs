use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let my_str = "hello";
    let my_string = String::from(my_str);

    // custom `From` trait
    let num = Number::from(30);
    println!("My number is {:?}", num);

    // using `into`
    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}