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
    // 调用宏 my_macro。宏是以 ! 结尾的，是在 Rust 中区分宏和普通函数调用的方式。
    my_macro!();
}
