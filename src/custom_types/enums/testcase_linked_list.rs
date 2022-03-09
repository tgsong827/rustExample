use std::fmt::format;

use super::print_current_module_path;
use List::*;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has lenght: {}", list.len());
    println!("{}", list.stringify());
}

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    // 빈 리스트 생성
    fn new() -> List {
        Nil
    }

    // list를 취하고 동일 list와 새 요소를 전면에 추가해 반환.
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // list의 길이 반환.
    fn len(&self) -> u32 {
        match *self {
            // tail에 대한 소유권을 얻을 수 없는 이유는 `self`가 대여중이기 때문.
            // 대신 tail에 대한 참조를 빌린다?.
            Cons(_,ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
   
    // list를 string으로 표현한 것(heap 할당).
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}