use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    // 정수 더하기
    println!("1 + 2 = {}", 1u32 + 2);

    // 정수 빼기
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ `1i32`를 `1u32`로 변경하여 왜 타입이 중요한지 알아보세요.

    // 짧은 boolean 논리 연산
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 비트 연산
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 밑줄을 사용하여 가독성 올려버리기~
    println!("One million is written as {}", 1_000_000u32);
}