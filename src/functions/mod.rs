use super::print_current_module_path;

pub mod methods;
pub mod closures;
pub mod higher_order_functions;
pub mod diverging_functions;

pub fn execute_example() {
    print_current_module_path(module_path!());

    // C나 C++ 처럼 함수의 정의 순서는 중요하지 않다.
    // 여기서 맨 아래 정의 되어 있는 fizzbuzz_to 함수를 사용할 수 있다.
    fizzbuzz_to(100);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // 초기 반환 케이스
    if rhs == 0 {
        return false;
    }

    // 리턴을 의미하는 표현으로 `return` 키워드가 필요없다.
    lhs % rhs == 0
}

// 리턴하지 않는 함수, 실제론 unit type 인 `()` 를 반환하는 것이다
// void와 비슷한 것??
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

//`()` 를 반환할때, return type은 생략될 수 있다.
// void와 비슷한 것??
fn fizzbuzz_to(n: u32) {
    for n in 1..n {
        fizzbuzz(n);
    }
}