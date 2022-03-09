#![allow(overflowing_literals)]

use std::u8;
use super::print_current_module_path;

pub mod alias;
pub mod inference;
pub mod literals;

pub fn execute_example() {
    print_current_module_path(module_path!());

    // 기본 타입간의 변환은 암시없는 타입 변환.
    // 명시적 타입 변환은 `as` 키워드 사용.

    let decimal = 65.4321_f32;

    // Error: 암시적 변환 안됨
    // let integer: u8 = decimal;

    // 명시적 형변환
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is: {}", 1000 as u8);
    println!(" -1 as a u8 is: {}", (-1i8) as u8);

    println!("1000 mod 256 is : {}", 1000 % 256);

    println!("128 as a i16 is: {}", 128 as i16);
    println!("128 as a i8 is : {}", 128 as i8);

    println!("1000 as a u8 is : {}", 1000 as u8);
    println!("232 as a i8 is : {}", 232 as i8);

    // Since Rust 1.45
    println!("300.0 is {}", 300.0_f32 as u8); // 최대값 255
    println!("-100.0 as u8 is {}", -100.0_f32 as u8); // 최소값 0
    println!("nan as u8 is {}", f32::NAN as u8); // nan as u8 is 0

    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}