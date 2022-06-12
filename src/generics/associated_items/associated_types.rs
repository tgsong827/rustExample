use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());


}

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

// fn difference<A, B, C>(container: &C) -> i32 where C: Contains<A, B> { ... }

// trait Contains 처럼 type 을 활용하면 더이상 A,B 를 표시할 필요가 없어진다.
// fn difference<C: Contains>(container: &C) -> i32 { ... }

