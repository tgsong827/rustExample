use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    // 문장

    // 변수 바인딩
    let x = 5;

    // 표현
    x;
    x + 1;
    15;

    // block
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 이 표현은 `y`ㅇ에 값을 대입한다.
        x_cube + x_squared + x
    };

    let z = {
        // 이 세미콜론 문은 해당 문장을 끝맺고 `()`을 `z`에 대입한다.
        2 * x;
    };
}