use super::print_current_module_path;
use std::fmt::Debug;

pub fn execute_example() {
    print_current_module_path(module_path!());
    
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T where Option<T> : Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}