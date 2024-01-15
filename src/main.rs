#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use learn_os::{asm::hlt_loop, println};
use x86_64::registers::control::Cr3;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info:#?}");
    hlt_loop()
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    learn_os::init();

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table: {:#?}",
        level_4_page_table.start_address()
    );

    println!("Did not crash!");
    hlt_loop()
}
