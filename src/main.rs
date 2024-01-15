#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use learn_os::{asm::hlt_loop, println};

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info:#?}");
    hlt_loop()
}

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    learn_os::init(boot_info);

    println!("Did not crash!");
    hlt_loop()
}
