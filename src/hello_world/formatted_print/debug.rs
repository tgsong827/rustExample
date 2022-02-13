use std::fmt::Display;

use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    // let unPrintable = UnPirntable(3);
    // println!("print UnPrintable struct : {:?}", unPrintable);

    let debugPrintable = DebugPrintable(5);
    println!("print DebugPrintable struct : {:?}", debugPrintable);

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
            actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));

}


struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);