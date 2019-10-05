#![feature(global_asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"));

mod printer;

#[no_mangle]
pub extern "C" fn __start_rust() -> ! {
    println!("Hello World{}", "!");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // XXX doesn't work for some reasons
    //println!("{}", info);
    loop {}
}

#[no_mangle]
pub fn abort() -> ! {
    loop {}
}
