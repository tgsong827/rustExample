use super::print_current_module_path;

pub mod capturing;
pub mod as_input_parameters;
pub mod type_anonymity;
pub mod input_functions;
pub mod as_output_parameters;

pub fn execute_example() {
    print_current_module_path(module_path!());

    fn function(i: i32) -> i32 { i + 1 }

    // closure는 `{}` 중괄호 바디를 쓰는 것은 선택적 사항이다.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    // 타입 명시가 생력되어 있으면 가장 빨리 사용하는 곳에 의해 추론된다.
    // 이 경우엔 아래의 i 변수가 될 것이다.
    let closure_inferred = |i| i + 1;

    let i = 1;

    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // `i32`를 반환하는 인자를 취하지 않는 클로저
    // 반환 타입은 추론된다.
    let one = || 1;
    println!("closure returning one: {}", one());

}