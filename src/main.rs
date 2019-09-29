#![feature(global_asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"));

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
//pub extern "C" fn _start() -> ! {
pub extern "C" fn __start_rust() -> ! {
    /*
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    */
    let uart = 0x1001_3000  as *mut u8;
    for c in b"Hello from Rust!".iter() {
        unsafe {
            *uart = *c as u8;
        }
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
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
