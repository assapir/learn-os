#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader::BootInfo;

pub mod asm;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod vga_buffer;

pub fn init(boot_info: &'static BootInfo) {
    gdt::init();
    interrupts::init();
    memory::init(boot_info);
}
