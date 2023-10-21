// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 在其定义的位置插入了宏生成的代码
    my_macro!();
}

// 这段Rust代码定义了一个宏（macro）和一个主函数。我会逐行为你解释这段代码的含义。

// macro_rules! my_macro {：这行定义了一个名为my_macro的宏。在Rust中，宏是一种元编程工具，用于生成代码在编译之前。宏可以接受参数并在编译时生成代码。
// () => {：这是宏的定义开始部分。在这里，()表示这个宏没有任何参数。=>是分隔符，将左边的参数列表和右边的代码块分隔开。
// println!("Check out my macro!");：这是宏的主体部分，它使用println!宏来输出"Check out my macro!"这句话。这里使用了Rust的格式化字符串功能。
// };：这个分号表示宏的定义结束。
// fn main() {：这行定义了主函数（main函数），它是程序的入口点。
// my_macro!();：这行代码在主函数中调用了先前定义的my_macro宏，并在其定义的位置插入了宏生成的代码。因此，当程序运行时，它会输出"Check out my macro!"。
// }：这个分号表示主函数的结束。
// 总结：这段代码定义了一个简单的宏，当它被调用时，会输出"Check out my macro!"。在这个例子中，你直接在主函数中调用了这个宏，因此当你运行这个程序时，你会在控制台看到这句话。