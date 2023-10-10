use crate::{erro, gdt, info, okay, print, warn};
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub const KEYBOARD_PORT: u16 = 0x60;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub static mut TIMER_TICKS: spin::RwLock<u128> = spin::RwLock::new(0);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum IntIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl From<IntIndex> for usize {
    fn from(value: IntIndex) -> Self {
        u8::from(value) as usize
    }
}

impl From<IntIndex> for u8 {
    fn from(value: IntIndex) -> Self {
        value as u8
    }
}

lazy_static::lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_h);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_h).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.page_fault.set_handler_fn(page_fault_h);
        idt[IntIndex::Timer.into()].set_handler_fn(timer_h);
        idt[IntIndex::Keyboard.into()].set_handler_fn(keyboard_h);
        idt
    };
}

pub fn init() {
    info!("loading idt");
    IDT.load();
    okay!("loaded idt");
}

pub fn wait_int() {
    x86_64::instructions::hlt();
}

pub fn idle_mode() -> ! {
    loop {
        wait_int();
    }
}

extern "x86-interrupt" fn breakpoint_h(stack_frame: InterruptStackFrame) {
    warn!("breakpoint exception");
    info!("\tstack frame: {stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_h(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    erro!("double fault exception");
    info!("\terror code: {error_code}");
    info!("\tstack frame: {stack_frame:#?}");
    panic!("double fault exception");
}

extern "x86-interrupt" fn page_fault_h(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    warn!("page fault exception");
    info!("\terror code: {error_code:?}");
    info!("\tacessed address: {:?}", Cr2::read());
    info!("\tstack frame: {stack_frame:#?}");
    idle_mode();
}

extern "x86-interrupt" fn timer_h(_stack_frame: InterruptStackFrame) {
    let mut timer_w = unsafe { TIMER_TICKS.write() };
    *timer_w += 1;
    unsafe { PICS.lock().notify_end_of_interrupt(IntIndex::Timer.into()) }
}

extern "x86-interrupt" fn keyboard_h(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;

    lazy_static::lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(ScancodeSet1::new(), layouts::Us104Key,HandleControl::Ignore)
        );
    }

    let mut kb = KEYBOARD.lock();
    let mut port = Port::new(KEYBOARD_PORT);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = kb.add_byte(scancode) {
        if let Some(key) = kb.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(ch) => print!("{ch}"),
                DecodedKey::RawKey(rk) => print!("{rk:?}"),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(IntIndex::Keyboard.into())
    }
}

pub fn get_ticks() -> u128 {
    unsafe { *TIMER_TICKS.read() }
}
