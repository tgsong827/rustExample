use super::print_current_module_path;

type NanoSecond = u64;
type Inch = u64;

// ** Type들은 반드시 CamelCase 식의 이름을 지어야 한다.
#[allow(non_camel_case_types)]
type u64_t = u64;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!("{} nanoseconds + {} inches = {} unit?", 
            nanoseconds, 
            inches, 
            nanoseconds + inches);

    // 타입을 사용하는 주된 이유는 타이핑을 줄이기 위함.
    // 예를 들어, IoResult<T> 타입은 Result<T, IoError> 타입의 별칭이다.
}