// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

// 这是一个非常典型的递归类型问题的例子，你需要使用Box来解决问题。Rust中的Box可以用来在堆上分配内存，这样就可以解决递归类型的问题，因为你可以有一个指向下一个元素（也是相同类型的）的指针。

// 以下是修复你的代码的步骤：

// 首先，我们需要在List枚举类型中添加一个Box。这意味着我们需要把List的定义改为Box<List>。但是，这里有一个问题，Box只能用于存储一种类型，而你的List枚举类型中有两种类型：Cons和Nil。为了解决这个问题，你可以创建一个新的枚举类型，只包含一种类型，然后用Box来存储这个枚举类型。


#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    // todo!()
    List::Nil
}

pub fn create_non_empty_list() -> List {
        let tail = Box::new(List::Nil);
    List::Cons(42, tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
