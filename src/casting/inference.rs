use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let elem = 5u8;

    // 이 시점에는 벡터의 타입을 알지 못한다,
    let mut vec = Vec::new();

    // 백터의 타입이 결정된다.
    vec.push(elem);

    println!("{:?}", vec);
}