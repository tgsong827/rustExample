use super::print_current_module_path;
use std::fmt;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

// `Vec`를 보관하는 `List`란 이름의 구조체 정의.
struct List(Vec<i32>);

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let List(ref vec) = *self;

//         write!(f,"[")?;

//         for (count, v) in vec.iter().enumerate() {
//             if count != 0 { 
//                 write!(f,", ")?; 
//             }
//             write!(f,"{}", v)?;
//         }

//         write!(f,"]")
//     }
// }

// Activity
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec ) = *self;

        write!(f,"[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f,", ")?;
            }
            write!(f,"{}: {}", count, v)?;
        }

        write!(f,"]")
    }
}