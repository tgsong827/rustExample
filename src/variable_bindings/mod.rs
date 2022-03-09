use super::print_current_module_path;

pub mod mutability;
pub mod scope_and_shadowing;
pub mod declare_first;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // `an_integer`가 복사 된다.
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);
   
    // 접두어로 `_` 를 넣으면 사용되지 않는 변수에 대한 경고가 사라짐 
    let _unused_variable = 3u32;
    let noisy_unused_variable = 2u32;
}