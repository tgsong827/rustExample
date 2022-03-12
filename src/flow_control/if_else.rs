use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // 이 표현은 `i32`를 반환한다.
            10 * n
        } else {
            println!(", and is a big number, reduce by two");

            n / 2
        };

    println!("{} -> {}", n, big_n);
}