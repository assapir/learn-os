#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use learn_os::{asm::hlt_loop, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info:#?}");
    hlt_loop()
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    learn_os::init();

    println!("Did not crash!");
    hlt_loop()
}
