use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let open_box = my::OpenBox { contents: "public information" };

    println!("The open box contains: {}", open_box.contents);

    // ClosedBox의 contents 변수는 private 가시성이다.
    // let closed_box = my::ClosedBox { contents: "classified information" };

    let _closed_box = my::ClosedBox::new("classified information");

    // ClosedBox의 contents 변수는 private 가시성이다.
    // println!("The closed box contains: {}", _closed_box.contents);
}

mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { 
                contents: contents,
            }
        }
    }
}