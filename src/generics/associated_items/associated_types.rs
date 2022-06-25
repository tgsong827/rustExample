use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

struct Container(i32, i32);

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

// fn difference<A, B, C>(container: &C) -> i32 where C: Contains<A, B> { ... }

// trait Contains 처럼 type 을 활용하면 더이상 A,B 를 표시할 필요가 없어진다.
fn difference<C: Contains>(container: &C) -> i32 { 
    container.last() - container.first()
 }

