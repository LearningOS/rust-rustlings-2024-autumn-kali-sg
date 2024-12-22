// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.



#[rustfmt::skip]
macro_rules! my_macro {
    () => {  // 有参数宏
        println!("Check out my macro!");
    };   // 不同宏之间需要添加一个分号

    ($val:expr) => {  // 无参数宏
        println!("Look at this other macro: {}", $val);
    }
    /* Rust 编译器按照定义顺序依次尝试匹配宏规则。
     * 对于每一个宏调用，编译器会从上到下查找最匹配的规则，并使用该规则展开宏。
     * 如果有多条规则都匹配，则优先选择最早定义的那一条。 */
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
