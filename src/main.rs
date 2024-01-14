#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use learn_os::{asm::hlt, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info:#?}");
    hlt()
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    learn_os::init();

    println!("Did not crash!");
    hlt()
}
