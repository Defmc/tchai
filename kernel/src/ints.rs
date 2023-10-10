use crate::{erro, gdt, info, okay, warn};
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub static mut TIMER_TICKS: u128 = 0;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum IntIndex {
    Timer = PIC_1_OFFSET,
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
        idt[IntIndex::Timer.into()].set_handler_fn(timer_h);
        idt
    };
}

pub fn init() {
    info!("loading idt");
    IDT.load();
    okay!("loaded idt");
}

extern "x86-interrupt" fn breakpoint_h(stack_frame: InterruptStackFrame) {
    warn!("breakpoint exception");
    info!("stack frame: {stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_h(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    erro!("double fault exception");
    info!("\terror code: {error_code}");
    info!("\tstack frame: {stack_frame:#?}");
    panic!("double fault exception");
}

extern "x86-interrupt" fn timer_h(_stack_frame: InterruptStackFrame) {
    unsafe { TIMER_TICKS += 1 };
    unsafe { PICS.lock().notify_end_of_interrupt(IntIndex::Timer.into()) }
}
