//! 用户程序库
//!
//! 提供了系统调用的方便接口，供运行在用户态的 Rust 程序使用。
//! 当前支持 `write` 和 `exit` 两个系统调用。
//!
//! 所有系统调用的底层实现来自 `syscall` 模块，本库对外暴露
//! 简化过的 API，使其更容易从用户程序直接调用。

#![no_std]

// 启用弱链接特性，允许定义可被外部覆盖的 main 函数
#![feature(linkage)]

#[macro_use]
pub mod console;
mod syscall;
mod lang_items;

/// 清零 BSS 段
///
/// 读取链接脚本提供的 `start_bss` 和 `end_bss` 符号，
/// 将该范围内的内存全部写入 0，避免未初始化的静态变量
/// 包含随机值。
fn clear_bss() {
    unsafe extern "C" {
        fn start_bss();
        fn end_bss();
    }
    let start_bss_ptr = start_bss as *const () as usize;
    let end_bss_ptr = end_bss as *const () as usize;
    (start_bss_ptr..end_bss_ptr).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/// 用户程序入口点
///
/// 被放置在 `.text.entry` 段内，确保是最先执行的代码。
/// 首先清零 BSS 段，然后调用用户提供的 `main` 函数，
/// 最后通过 `exit` 系统调用结束进程。
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

/// 默认的 main 函数
///
/// 使用弱链接 (`#[linkage = "weak"]`) 定义，允许用户在
/// 自己的 Rust 源文件中覆盖该函数。如果用户没有提供
/// 自己的 `main`，该默认实现会触发 panic，提示缺少 main。
#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("Cannot find main!");
}
// 导入 syscall 模块中的所有公开符号，
// 以便后续定义的 write 和 exit 可以直接调用 sys_write、sys_exit。
use syscall::*;

/// 向文件描述符写入数据
///
/// 这是一个对 `sys_write` 系统调用的包装，将缓冲区中的数据写入
/// 到指定的文件描述符。
///
/// # 参数
///
/// * `fd` - 文件描述符，0 为标准输入，1 为标准输出，2 为标准错误
/// * `buf` - 需要写入的字节缓冲区
///
/// # 返回值
///
/// 返回实际写入的字节数；若出错则返回一个负数错误码
pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }

/// 退出当前进程
///
/// 包装 `sys_exit` 系统调用，使进程以给定的状态码退出。
/// 该函数通常不会返回，若返回则表示系统调用失败。
///
/// # 参数
///
/// * `exit_code` - 进程的退出码，0 表示正常退出
///
/// # 返回值
///
/// 正常情况下不会返回；若返回则可能为一个负数错误码
pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code) }
