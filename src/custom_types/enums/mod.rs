use super::print_current_module_path;

pub mod use_;
pub mod c_like;
pub mod testcase_linked_list;


pub fn execute_example() {
    print_current_module_path(module_path!());
    
    let person  = Person::Height(18);
    let amira   = Person::Weight(10);
    let dave    = Person::Info { name: "Dave".to_owned(), height: 72 };
    let rebecca = Person::Scientist;
    let rohan   = Person::Enginner;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}

enum Person {
    Enginner,
    Scientist,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 }
}

fn inspect(p: Person) {
    match p {
        Person::Enginner => println!("Is an engineer!"),
        Person::Scientist => println!("Is a scientist!"),
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}