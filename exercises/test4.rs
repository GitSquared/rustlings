// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!


#[macro_use]
mod macros {
    macro_rules! my_macro {
        ($val:expr) => {
            "Hello ".to_owned() + $val
        };
    }
}

fn main() {
    println!("{}", my_macro!("world!"));

    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
