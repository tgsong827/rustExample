use super::print_current_module_path;

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

pub fn execute_example() {
    print_current_module_path(module_path!());

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // 순차적 명령구문 접근법
    // 누석 변수 선언
    let mut acc = 0;
    // 반복: 0, 1, 2, ... 무한대까지
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                // 모든 자연수를 제곱
             .take_while(|&n| n < upper)    // 상한선 이하
             .filter(|n| is_odd(*n))       // 홀수라면 
             .fold(0, |sum, i| sum + i);             // 이들의 합
    
    println!("functional style: {}", sum_of_squared_odd_numbers);
}