#![feature(panic_implementation)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate test_os;
extern crate x86_64;

use core::panic::PanicInfo;
use test_os::*;

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    hang()
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {

    x86_64::instructions::int3();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    hang()
}
