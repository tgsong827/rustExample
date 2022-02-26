use std::{str::MatchIndices, fmt::{self, *}};

use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    // 서로 다른 타입 무리의 튜플
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                                        -1i8, -2i16, -3i32, -4i64,
                                        0.1f32, 0.2f64, 'a', true);
    
    // tuple에서 색인으로 값을 추출 할 수 있다.
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 튜플이 튜플의 멤버가 될 수 있다.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 튜플은 출력 가능
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // 하나의 요소인 튜플을 만드려면, 괄호와는 별도로 쉼표를 통해 알리는게 필요하다.
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 튜플은 바인딩을 생성해서 역구조화 할 수 있다.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{}", transpose(matrix));
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn transpose(pair: Matrix) -> Matrix {
    // let matrix = pair;

    Matrix(pair.0, pair.2, pair.1, pair.3)
}