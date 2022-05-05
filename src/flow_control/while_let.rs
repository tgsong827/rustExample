use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let mut optional = Some(0);

    // awkward match sequences
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            },
            _ => { break; }
        }
    }

    optional = Some(0);

    // Using `while let` makes this sequence much nicer.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}