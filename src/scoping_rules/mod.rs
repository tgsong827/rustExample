use super::print_current_module_path;

pub mod raii;
pub mod borrowing;
pub mod lifetimes;
pub mod ownership_and_moves;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4i32);

        // `_box3`는 여기서 해제
    }

    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2`는 여기서 해제
}

fn create_box() {
    let _box1 = Box::new(3i32);

    // `_box1`은 여기서 파괴되고, 메모리는 해제된다.
}