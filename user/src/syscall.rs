//! 用户态系统调用封装
//!
//! 使用 RISC-V 指令集中的 `ecall` 指令触发内核系统调用，
//! 提供了 `write` 和 `exit` 两个常用系统调用的便捷函数。

use core::arch::asm;

/// 系统调用号：`write`，对应 Linux syscall 64
const SYSCALL_WRITE: usize = 64;
/// 系统调用号：`exit`，对应 Linux syscall 93
const SYSCALL_EXIT: usize = 93;
/// 系统调用号：`yield`，对应 Linux syscall 124
const SYSCALL_YIELD: usize = 124;

/// 执行一次系统调用
///
/// 通过 RISC-V 的 `ecall` 指令将系统调用号和参数传入内核，
/// 并将内核返回的结果存放在 `a0` 寄存器中。
///
/// # 参数
///
/// * `id` - 系统调用号
/// * `args` - 最多三个参数，分别保存到 `a0`、`a1`、`a2` 寄存器
///
/// # 返回值
///
/// 内核在 `a0` 寄存器中返回的 isize 类型值。
///
/// # 注意事项
///
/// 该函数内部包含内联汇编，调用者需要保证传入的参数符合 RISC-V 系统调用约定。
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        // 执行 ecall 指令并传递参数
        asm!("ecall",
             in("x10") args[0],
             in("x11") args[1],
             in("x12") args[2],
             in("x17") id,
             lateout("x10") ret
        );
    }
    ret
}

/// 向指定文件描述符写入数据
///
/// 封装了 `write` 系统调用，将缓冲区中的数据写入到文件中。
///
/// # 参数
///
/// * `fd` - 文件描述符（例如标准输出 stdout 为 1）
/// * `buffer` - 要写入的字节序列
///
/// # 返回值
///
/// 返回实际写入的字节数（出错时为负数）
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

/// 退出当前进程并返回退出码
///
/// 封装了 `exit` 系统调用，进程正常退出后将不会返回。
///
/// # 参数
///
/// * `exit_code` - 退出状态码
///
/// # 返回值
///
/// 正常情况下不会返回；若返回则可能表示系统调用失败
pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

