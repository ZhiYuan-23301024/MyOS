/*
 * 链接脚本：用户程序
 *
 * 本脚本定义用户程序在内存中的布局。程序被加载到地址 BASE_ADDRESS 处，
 * 并将输入的各段（text、rodata、data、bss）按顺序放置，同时提供
 * bss 段的起始/结束符号，供引导代码清零 bss 区域。
 */

OUTPUT_ARCH(riscv)            /* 目标体系结构为 RISC‑V */
ENTRY(_start)                 /* 入口符号为 _start */

BASE_ADDRESS = 0x80400000;    /* 用户程序加载的基地址 */

SECTIONS
{
    /* 从基地址开始放置所有段 */
    . = BASE_ADDRESS;

    /* 代码段 */
    .text : {
        *(.text.entry)        /* 保证 _start 所在的小节最先出现 */
        *(.text .text.*)      /* 其余代码 */
    }

    /* 只读数据段 */
    .rodata : {
        *(.rodata .rodata.*)  /* 普通只读数据 */
        *(.srodata .srodata.*)/* 短只读数据 (RISC‑V small data) */
    }

    /* 可读写数据段 */
    .data : {
        *(.data .data.*)      /* 已初始化的全局/静态变量 */
        *(.sdata .sdata.*)    /* 短数据 (RISC‑V small data) */
    }

    /* BSS 段（未初始化或零初始化的全局/静态变量） */
    .bss : {
        start_bss = .;        /* bss 起始地址 */
        *(.bss .bss.*)
        *(.sbss .sbss.*)      /* 短 bss (RISC‑V small data) */
        end_bss = .;          /* bss 结束地址 */
    }

    /* 不需要保留的段，直接丢弃以节省空间 */
    /DISCARD/ : {
        *(.eh_frame)          /* 异常处理帧信息（对裸机无用） */
        *(.debug*)            /* 调试信息 */
    }
}
