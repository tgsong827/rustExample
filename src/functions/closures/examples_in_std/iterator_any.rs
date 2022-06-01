use super::print_current_module_path;

// pub trait Iterator {
//     type Item;

//     fn any<F>(&mut self, f: F) -> bool where 
//         F: FnMut(Self::Item) -> bool {}
// }

pub fn execute_example() {
    print_current_module_path(module_path!());

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}