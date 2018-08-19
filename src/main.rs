#![feature(panic_implementation)]
#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;

use core::panic::PanicInfo;

#[macro_use]
mod vga;


#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_println!("Test");
    vga_println!();
    vga_println!("WOW!!!\n");
    vga_println!("Works.");

    loop {}
}
