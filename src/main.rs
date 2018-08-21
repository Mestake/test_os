#![feature(panic_implementation)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate test_os;

use core::panic::PanicInfo;
use test_os::exit_qemu;

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_println!("Hello World{}", "!");
    serial_println!("Hello Host{}", "!");

    unsafe { exit_qemu(); }

    loop {}
}
