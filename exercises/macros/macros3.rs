// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// 用于指示 Rust 编译器在当前作用域内导入并启用指定模块中的宏定义。
#[macro_use]
// 当你定义了一个宏，并且希望它不仅限于当前 crate 内部使用，
// 是可供外部 crate 引用和调用时，就需要使用 #[macro_export] 属性。
// #[macro_export]
mod macros {
    macro_rules! zzh_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}
fn main() {
    zzh_macro!();
}
