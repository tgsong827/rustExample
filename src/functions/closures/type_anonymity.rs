use super::print_current_module_path;

// `F`는 반드시 입력이나 반환하지 않는 클로저에 대해 `Fn`을 구현해야 한다.
// 정확히 이는 `print`를 위해 필요하다.
fn apply<F>(f: F) where 
    F: Fn() {
    f();
}

pub fn execute_example() {
    print_current_module_path(module_path!());

    let x = 7;

    // `x`를 익명 타입으로 캡처하고 `Fn`을 구현한다. 이를 `print`에 저장.
    let print = || println!("{}", x);

    apply(print);
}