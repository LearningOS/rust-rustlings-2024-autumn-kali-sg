// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {  // 宏的定义必须在调用前面 
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
