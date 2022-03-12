#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn print_current_module_path(module_path: &str) {
    println!("\n####  Module Path - {}  ####\n", module_path);
}

pub mod hello_world;
pub mod primitives;
pub mod custom_types;
pub mod variable_bindings;
pub mod casting;
pub mod expressions;