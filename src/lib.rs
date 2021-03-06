#![feature(panic_implementation)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![no_std]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[cfg(test)]
extern crate std;

#[macro_use]
extern crate lazy_static;
extern crate bootloader;
extern crate spin;
extern crate volatile;
extern crate uart_16550;
extern crate x86_64;

#[cfg(test)]
extern crate array_init;

#[macro_use]
pub mod vga;
#[macro_use]
pub mod serial;
pub mod interrupts;
pub mod cpuio;
pub mod memory;

pub fn hang() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init() {
    memory::init();
    interrupts::init();
}

pub unsafe fn exit_qemu() -> ! {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);

    unreachable!("Exiting qemu failed")
}
