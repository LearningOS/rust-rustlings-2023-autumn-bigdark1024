// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// pub fn factorial(num: u64) -> u64 {
//     // Complete this function to return the factorial of num
//     // Do not use:
//     // - return
//     // Try not to use:
//     // - imperative style loops (for, while)
//     // - additional variables
//     // For an extra challenge, don't use:
//     // - recursion
//     // Execute `rustlings hint iterators4` for hints.
// }

// 这是一个求阶乘的问题，要求我们不能使用循环、额外的变量以及递归。
// 我们可以利用阶乘的定义：n的阶乘表示为n!，
// 是从1到n的所有正整数的乘积。
// 因此，我们可以通过迭代1到n来计算阶乘，而不使用任何禁止的技巧。
pub fn factorial(num: u64) -> u64 {
    let mut result = 1;
    for i in 1..= num {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
