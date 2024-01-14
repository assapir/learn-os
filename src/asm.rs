use core::arch::asm;

pub fn hlt() -> ! {
    unsafe {
        asm!("hlt");
    }
    loop {} // We don't really need that, only for types to be happy
}
