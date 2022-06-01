use super::print_current_module_path;

fn apply<F>(f: F) where F: FnOnce() {
    f()
}

fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}

pub fn execute_example() {
    use std::mem;
    print_current_module_path(module_path!());

    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();

    // 2개의 변수를 캡처한다. : `greeting`은 참조로 `farewell`은 값으로.
    let diary = || {
        // `greeting`은 참조에 의하므로 `Fn`이 필요하다.
        println!("I said {}.", greeting);

        // 변경은 `farewell`을 가변참조로 캡처되게 강제한다.
        // 여기서는 `FnMut`이 필요하다.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 수동으로 drop을 호출하면 `farewell`을 값으로 캡처되도록 강제한다.
        // 여기서는 `FnOnce`가 요구된다.
        mem::drop(farewell);
    };

    // 클로저를 적용하는 함수를 호출한다.
    apply(diary);

    // `double`은 `apply_to_3` trait의 범위를 만족시킨다.
    let double = |x| 2 * x;
    
    println!("3 doubled: {}", apply_to_3(double));
}
