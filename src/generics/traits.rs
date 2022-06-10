use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    // empty;
    // null;
}

struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}