//! 用户态 panic 处理模块
//!
//! 当用户程序触发不可恢复错误（如数组越界、断言失败、显式调用 `panic!` 宏等）时，
//! 本模块定义的 `#[panic_handler]` 会被调用。它负责将错误信息输出到控制台，
//! 然后陷入无限循环，从而终止出错的用户进程。
//!
//! 在当前环境中，`loop {}` 会使 CPU 在该处空转；真实操作系统会通过退出系统调用优雅地结束进程。

use core::panic::PanicInfo;

/// 用户程序 panic 处理器
///
/// 被标记为 `#[panic_handler]`，当程序发生 panic 时自动调用。
/// 首先尝试从 `panic_info` 中提取 panic 消息；若消息存在，则结合出错的位置（文件名、行号）
/// 打印出详细的错误信息；否则仅打印 `Panicked:` 后跟消息本身。
///
/// # 参数
///
/// * `panic_info` - 包含触发 panic 的信息，如消息、文件名、行号等
///
/// # 行为
///
/// 调用 `println!` 将错误信息输出到控制台，然后进入无限循环，函数永不返回。
#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    let msg = panic_info.message().as_str().unwrap_or("(no message)");

    if let Some(location) = panic_info.location() {
        println!(
            "Panicked at {}:{}, {}",
            location.file(),
            location.line(),
            msg
        );
    } else {
        println!("Panicked: {}", msg);
    }
    // 无限循环，模拟进程终止
    loop {}
}
