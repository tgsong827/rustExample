pub fn execute_example() {
    // 일반적으로 `{}`는 인자에 따라 자동으로 변환된다.
    // 다음은 String으로 변환될 것이다.
    println!("{} days", 31);

    // 접미사가 없으면, 31은 i31이 된다.
    // 접미사를 추가해 31의 type을 변경할 수 있다.

    // 출력을 위한 다양한 옵션들이 있다.
    // 위치지정 인자도 사용될 수 있다.
    println!("{0}, this is {1}. {1}, this is {0}","Alice", "Bob");

    // 이름을 인자로 사용할 수 있다.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // `:`; 뒤에 특수 형식 지정자를 사용할 수 있다..
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 넓이를 지정하여 오른쪽 정렬을 사용할 수 있다. 이는 다음과 같이 출력될 것이다.
    // "    1". 5칸의 공백과 "1".
    println!("{number:>width$}", number=1, width=6);

    // 여분의 공간을 0으로 채운 숫자도 사용할 수 있다. 이는 "000001"을 출력할 것.
    println!("{number:>0width$}", number=1, width=6);

    // 위치지정 인자 사용시 정확한 수의 인자들이 왔는데 검증받게 될 것이다.
    //println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ 다음의 인자를 추가해서 버그를 수정해보세요 : "James"

    // `i32` 내장한 구조체를 만들자. 그리고 `Structure`라고 이름지었다.
    #[allow(dead_code)]
    struct Structure(i32);

    // 하지만, Structure와 같이 당신이 만든 형태는 좀더 복잡해진다.
    // 다음 문장은 실행되지 않을 것이다.
    //println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ 이 줄을 주석처리 해주세요.

    // Activities
    // let pi = 3.141592로 pi를 선언하고 Pi is roughly 3.142를 출력하는 println! 문을 추가해보세요.
    let pi = 3.141592;
    println!("Pi is roughly {:.3}",pi);
}

//pub mod debug;
//pub mod display;