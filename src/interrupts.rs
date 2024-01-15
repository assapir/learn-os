use crate::{asm::hlt_loop, gdt, print, println};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::{
    instructions::port::Port,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX)
        };
        idt[InterruptIndex::Timer.into()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.into()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore
        ));
}

pub fn init() {
    init_idt();
    init_pics();
    x86_64::instructions::interrupts::enable();
}

fn init_pics() {
    unsafe {
        PICS.lock().initialize();
    }
}

fn init_idt() {
    IDT.load()
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl From<InterruptIndex> for u8 {
    fn from(value: InterruptIndex) -> Self {
        value as u8
    }
}

impl From<InterruptIndex> for usize {
    fn from(value: InterruptIndex) -> Self {
        usize::from(<InterruptIndex as Into<u8>>::into(value))
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    println!("EXCEPTION: DOUBLE FAULT\n{stack_frame:#?}");
    hlt_loop()
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    end_interrupt(InterruptIndex::Timer.into());
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    let mut keyboard = KEYBOARD.lock();
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                pc_keyboard::DecodedKey::RawKey(key) => print!("{key:?}"),
                pc_keyboard::DecodedKey::Unicode(character) => print!("{character}"),
            }
        }
    }

    end_interrupt(InterruptIndex::Keyboard.into());
}

fn end_interrupt(interrupt_id: u8) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(interrupt_id);
    };
}
