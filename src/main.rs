#![feature(global_asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"));

mod vga_buffer;

#[no_mangle]
//pub extern "C" fn _start() -> ! {
pub extern "C" fn __start_rust() -> ! {
    println!("Hello World{}", "!");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub fn abort() -> ! {
    loop {}
}

/*
#[cfg(target_arch = "riscv32")]
#[link_section = ".boot"]
global_asm!(r#"
_start:
    /* Set up stack pointer. */
    lui     sp, %hi(stack_end)
    ori     sp, sp, %lo(stack_end)

    /* Now jump to the rust world; __start_rust.  */
    j       __start_rust

.bss

stack_start:
    .skip 1024
stack_end:
"#);
*/
