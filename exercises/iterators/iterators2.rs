// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // vec![] 默认空向量注释掉
    let v2 = words.iter().map(|x| capitalize_first(x)).collect();
    v2
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|x| capitalize_first(x)).collect()
}

// 逐行解析这段代码：

// pub fn capitalize_words_string(words: &[&str]) -> String {
// 这行定义了一个公开的函数capitalize_words_string，它接受一个字符串切片数组作为参数（&[&str]），其中每个字符串都是通过引用传递的。该函数返回一个String类型的结果。
// 2. words.iter().map(|x| capitalize_first(x)).collect()

// 这行代码是函数的核心部分。它首先调用iter()方法来遍历数组中的每个字符串。然后，它使用map()方法来对每个字符串应用capitalize_first函数。capitalize_first函数将每个字符串的首字母转换为大写，而其他字母保持不变。最后，使用collect()方法将处理过的字符串收集到一个新的String中。
// 3. }

// 这行代码表示函数定义的结束。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
