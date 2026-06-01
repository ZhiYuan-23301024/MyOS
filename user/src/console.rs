//! 用户态控制台输出模块
//!
//! 提供格式化输出的宏 (`print!`, `println!`) 以及对底层写系统调用的封装，
//! 使 Rust 风格的控制台打印可以在用户程序中方便使用。

use core::fmt::{self, Write};
use super::write;

/// 标准输出（stdout）的文件描述符，值为 1
const STDOUT: usize = 1;

/// 标准输出的简易实现
///
/// 实现 `core::fmt::Write` trait，将格式化后的字符串通过
/// 系统调用写入标准输出。
struct Stdout;

impl Write for Stdout {
    /// 将格式化后的字符串按字节写入标准输出
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

/// 将格式化参数输出到标准输出
///
/// 内部使用 `Stdout` 对象执行格式化写入，丢弃 IO 错误。
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// 格式化输出宏，不自动追加换行
///
/// 用法与 Rust 标准库中的 `print!` 一致。
///
/// # 示例
///
/// ```
/// print!("Hello, {}", "world");
/// ```
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// 格式化输出宏，自动追加换行
///
/// 用法与 Rust 标准库中的 `println!` 一致。
///
/// # 示例
///
/// ```
/// println!("Hello, {}", "world");
/// ```
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
