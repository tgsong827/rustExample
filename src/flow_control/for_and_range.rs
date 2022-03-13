use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    // ## For And Range ##
    // 1 ~ 100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 1 ~ 100
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // ## For And Iterators ##
    let names = vec!["Bob", "Frank", "Ferris"];

    // iter()
    for name in names.iter() {
        match name {
            &"Ferrirs" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    // into_iter() : 한번 호출 되면 값 자체가 소비 되고, 'moved' 되어 names는 다시 사용될 수 없음
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}" , name),
        } 
    }
    // println!("names: {:?}", names);

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
    
}