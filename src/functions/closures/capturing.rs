use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());
    use std::mem;

    // ------------------------------------------------------
    // closure로 immutable 변수가 이동 한다.
    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;
    // ------------------------------------------------------



    // ------------------------------------------------------
    // closure로 mutable 변수가 이동 한다.
    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();

    // let _reborrow = &count;
    inc(); // 마지막 closure 코드
    let _count_reborrowed = &mut count; // 마지막 closure
    // ------------------------------------------------------



    // ------------------------------------------------------
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // consume();
    // ------------------------------------------------------


    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // `move` 를 제거하면 정상 동작할 것이다.
    // println!("There're {} elements in vec", haystack.len());

}