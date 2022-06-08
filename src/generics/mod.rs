use super::print_current_module_path;

pub mod functions;
pub mod implementation;
pub mod traits;
pub mod bounds;
pub mod multiple_bounds;
pub mod where_clauses;
pub mod new_type_idiom;
pub mod associated_items;
pub mod phantom_type_parameters;

pub fn execute_example() {
    print_current_module_path(module_path!());

}

// A concrete Type `A`.
struct A;

struct Single(A);

struct SingleGen<T>(T);

fn main() {
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}