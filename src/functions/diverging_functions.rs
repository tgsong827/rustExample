use super::print_current_module_path;

fn foo() -> ! {
    panic!("This call never returns");
}

fn some_fn() {
    ()
}

pub fn execute_example() {
    print_current_module_path(module_path!());

    let a: () = some_fn();
    println!("This function returns and you can see this line.");

    // ....
    // 아직 진행중..
    // let x: ! = panic!("This call never returns.");
    // println!("You will never see this line!");
    
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i%2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }

    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}