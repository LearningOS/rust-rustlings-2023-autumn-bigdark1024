// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

macro_rules! my_macro {
    () => { // `()` 表示此宏不接受任何参数。
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
