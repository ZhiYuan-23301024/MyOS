#![no_std]
#![no_main]
#[macro_use]

mod console;
mod lang_items;
mod sbi;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as *const () as usize..ebss as *const () as usize)
        .for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    unsafe extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();
    println!("Hello, world!");
    println!(
        ".text [{:#x}, {:#x})",
        stext as *const () as usize,
        etext as *const () as usize
    );
    println!(
        ".rodata [{:#x}, {:#x})",
        srodata as *const () as usize,
        erodata as *const () as usize
    );
    println!(
        ".data [{:#x}, {:#x})",
        sdata as *const () as usize,
        edata as *const () as usize
    );
    println!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as *const () as usize,
        boot_stack_top as *const () as usize
    );
    println!(
        ".bss [{:#x}, {:#x})",
        sbss as *const () as usize,
        ebss as *const () as usize
    );
    println!("Hello, world!");
    panic!("Shutdown machine!");
}
#![no_std]
#![no_main]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod batch;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    let sbss_ptr = sbss as *const () as usize;
    let ebss_ptr = ebss as *const () as usize;
    (sbss_ptr..ebss_ptr).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[Kernel] Hello, world!");
    trap::init();
    batch::init();
    batch::run_next_app();
}

