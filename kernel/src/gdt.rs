use lazy_static::lazy_static;
use x86_64::structures::{
    gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
    tss::TaskStateSegment,
};
use x86_64::VirtAddr;

use crate::{info, okay};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub struct SegSelectors {
    code: SegmentSelector,
    tss: SegmentSelector,
}

impl SegSelectors {
    pub fn new(code: SegmentSelector, tss: SegmentSelector) -> Self {
        Self { code, tss }
    }
}

pub fn init() {
    info!("loading gdt table");
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;
    GDT.0.load();
    info!("loaded gdt table");

    unsafe {
        info!("\tsetting registers for gdt");
        CS::set_reg(GDT.1.code);
        load_tss(GDT.1.tss);
        x86_64::instructions::segmentation::SS::set_reg(SegmentSelector::NULL);
        okay!("\tsetted registers for gdt");
    }
    okay!("gdt loaded");
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
    static ref GDT: (GlobalDescriptorTable, SegSelectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_seg = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_seg = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, SegSelectors::new(code_seg, tss_seg))
    };
}
