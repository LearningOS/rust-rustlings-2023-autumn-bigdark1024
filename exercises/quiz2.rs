// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 复用之前string1.rs例子
    fn trim_me(input: &str) -> String {
        // TODO: Remove whitespace from both ends of a string!
        input.trim().to_string()
    }

    fn generate_bar_str(string: &str, ct: usize) -> String {
        let mut ret = String::from(string);
        for _ in 0..ct {
            ret += "bar";
        }
        ret
    }

    // TODO: Complete the function signature!
    // Vec<(String, Command)> 元组作为入参，生成Vec结构
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![]; // String Vector
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            let mut uppcase = string.to_uppercase();
            match command {
                Command::Uppercase => output.push(uppcase),
                Command::Trim => output.push(trim_me(&string)),
                Command::Append(usize) => output.push(generate_bar_str(&string, *usize)),
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // 使用my_module::transformer模块
    use crate::my_module::transformer; // use crate
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
