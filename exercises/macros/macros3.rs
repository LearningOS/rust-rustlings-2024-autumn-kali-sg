// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.



mod macros {
    #[macro_export] macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    /* 宏的作用域：
     * 默认情况下，Rust 中的宏作用域是有限的。如果一个宏是在某个模块内部定义的，那么它默认只能在这个模块及其子模块中使用。
     * 如果在一个外部模块中使用这个宏，需要确保它被正确地导出和引入。
     * #[macro_export] 属性：
     * 当为一个宏添加 #[macro_export] 属性时，编译器会将这个宏提升到 crate 的根命名空间。
     * 这个宏就像是在 crate 的最顶层定义的一样，因此可以在 crate 的任何地方直接调用，而无需指定模块路径。 */
}

fn main() {
    my_macro!();
}
