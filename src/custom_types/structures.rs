use std::sync::mpsc::Receiver;

use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let point: Point = Point { x: 0.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    // let 바인딩을 통해 재 구조화
    let Point { x: my_x, y: my_y } = point;

    println!("my_x: {}", my_x);

    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x},
        p2: point,
    };

    // 단위 구조체 초기화
    let _nil = Nil;

    // 튜플 구조체 초기화
    let pair = Pair(1, 0.1);

    println!("pair contains {} and {}", pair.0, pair.1);

    // 튜플 구조체 재구조화
    let Pair(integer, devimal ) = pair;
    println!("pair contains {} and {}", integer, devimal);

    // Activity
    println!("직사각형 면적 = {}",rect_area(_rectangle));

    // println!("원랙 Rectangle = {:?} ",  _rectangle);
}

// 유닛 구조체
struct Nil;

// 튜플 구조체
struct Pair(i32, f32);

// 두 필드를 가진 구조체
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// 구조체는 다른 구조체의 필드로 사용될 수 있음.
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// Activity
fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle { p1: Point { x: x1, y: y1}, p2: Point { x: x2, y: y2 } } = rectangle;
    (x1 - x2) * (y1 - y2)
}