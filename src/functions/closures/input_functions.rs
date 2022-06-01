use super::print_current_module_path;

// 클로저가 인자로 사용될 수 있듯이 함수도 가능하다.
// 하지만, 함수는 변수를 캡처할 수 없고, 클로저가 더 유연.
// 클로저를 인자로 취하는 함수는, 어던 함수든 클로저의 trait 바인딩을 만족할 수 있다면 인자로 사용 가능.

// 제네릭 `F`를 인자로 취하는 함수를 정의하고
// `Fn`으로 바인딩하고 그를 호출한다.
fn call_function<F: Fn()>(f: F){
    f()
}

// 입력으로 사용될 `Fn` 바인딩을 만족시키는 함수.
fn print() {
    println!("I'm a function!")
}

pub fn execute_example() {
    print_current_module_path(module_path!());

    let closure = || println!("I'm a closure!");

    call_function(closure);
    call_function(print);
}