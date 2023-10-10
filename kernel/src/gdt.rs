use lazy_static::lazy_static;
use x86_64::VirtAddr;
use x86_64::{
    instructions::{self, segmentation::Segment},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
};

use crate::{info, okay};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub struct SegSelectors {
    kcode: SegmentSelector,
    kdata: SegmentSelector,
    tss: SegmentSelector,
}

impl SegSelectors {
    pub fn new(kcode: SegmentSelector, kdata: SegmentSelector, tss: SegmentSelector) -> Self {
        Self { kcode, kdata, tss }
    }

    pub unsafe fn set_segmentations(&self) {
        use instructions::{segmentation, tables};
        segmentation::CS::set_reg(self.kcode);
        segmentation::DS::set_reg(self.kdata);
        segmentation::ES::set_reg(self.kdata);
        segmentation::FS::set_reg(self.kdata);
        segmentation::GS::set_reg(self.kdata);
        segmentation::SS::set_reg(self.kdata);
        tables::load_tss(self.tss);
    }
}

pub fn init() {
    info!("initializing gdt");
    info!("\tloading gdt table");
    GDT.0.load();
    okay!("\tloaded gdt table");

    info!("\tsetting registers for gdt");
    unsafe { GDT.1.set_segmentations() }
    okay!("\tsetted registers for gdt");
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
        let kcode_seg = gdt.add_entry(Descriptor::kernel_code_segment());
        let kdata_seg = gdt.add_entry(Descriptor::kernel_data_segment());
        gdt.add_entry(Descriptor::UserSegment(0));
        gdt.add_entry(Descriptor::user_code_segment());
        gdt.add_entry(Descriptor::user_data_segment());

        let tss_seg = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, SegSelectors::new(kcode_seg, kdata_seg, tss_seg))
    };
}
