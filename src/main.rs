#![no_std]
#![no_main]

use core::panic::PanicInfo;

use learn_os::{asm::hlt, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info:#?}");
    hlt()
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    hlt()
}
