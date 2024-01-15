#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod asm;
pub mod gdt;
pub mod interrupts;
pub mod vga_buffer;

pub fn init() {
    gdt::init();
    interrupts::init();
}
