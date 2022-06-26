use super::print_current_module_path;
use std::marker::PhantomData;

pub mod testcase_unit_clarification;

pub fn execute_example() {

    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // 에러
    // println!("_struct1 == _struct2 yields: {}", _tuple1 == _tuple2);
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
}

#[derive(PartialEq)]
struct PhantomTuple<A,B>(A,PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A,B> { first: A, phantom: PhantomData<B> }

// A에는 저장공간이 할당되지만, B엔 할당되지 않는다.
// 그러므로, B는 연산에 사용될 수 없다.