use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I dont't like letters. Let's go with an emoticon :)!");
    }

    ///////////
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
    
    // fix the following example
    let a = Fuu::Bar;

    // if Fuu::Bar == a {
    if let Fuu::Bar = a {
        println!("a is foobar");
    }

}

enum Fuu {Bar}

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}