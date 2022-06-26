use super::print_current_module_path;
use std::ops::Add;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

pub fn execute_example() {
    print_current_module_path(module_path!());

    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    // `+` 는 우리가 `Length<Unit>`를 위해 구현한 `add()` 메소드를 호출한다.
    // Length가 Copy를 구현하기 때문에 add()는 one_foot과 one_meter를 소비하지 않고, self와 rhs로 copy한다.
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // Error
    // let one_feter = one_foot + one_meter;
}