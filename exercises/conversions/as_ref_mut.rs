// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.



// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {   // 添加  T: AsRef<str> 特性 
    arg.as_ref().as_bytes().len()  // 得到字符的长度，而不是字节数
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
// 函数接收一个参数，并返回该参数中字符的数量，而不是其字节的长度。
// 由于 Rust 中的字符串是以 UTF-8 编码存储的，因此一个字符可能由多个字节组成。
// 用 AsRef<str> 作为参数的特性边界，这样就可以接受任何可以借用为 &str 的类型，比如 String 或 &String 。

fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()  // 得到字符的数量，而不是字节数
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
use std::ops::{Mul, MulAssign};
fn num_sq<T /*:Mul<Output = T> + MulAssign + Copy*/>(arg: &mut Box<T>)  // 套壳Box<>添加了一重指针，在使用值时需要使用**
where
    T: MulAssign + Mul<Output = T> + Copy, // 确保 T 可以与自身相乘，结果也是 T 类型，并且 T 可以被复制
{
    // TODO: Implement the function body.
    **arg *= **arg ;   // ** 解引用，* 解引用，& 解引用，Box 解引用
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
