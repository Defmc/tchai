use x86_64::{structures::paging::PageTable, PhysAddr, VirtAddr};

use crate::{erro, info};

pub unsafe fn active_level_4_table(phys_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;
    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let virt = phys_mem_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

pub unsafe fn virt_to_phys(addr: VirtAddr, phys_mem_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::page_table::FrameError;

    let (mut l4_frame, _) = Cr3::read();
    let table_idxs = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];

    for &index in &table_idxs {
        let virt = phys_mem_offset + l4_frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        let entry = &table[index];
        l4_frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => {
                erro!("frame error (not present)");
                info!("l4 frame: {l4_frame:?}");
                info!("indexes: {table_idxs:?}");
                info!("offset: {phys_mem_offset:?}");
                return None;
            }
            Err(FrameError::HugeFrame) => {
                erro!("frame error (huge frame)");
                info!("l4 frame: {l4_frame:?}");
                info!("indexes: {table_idxs:?}");
                info!("offset: {phys_mem_offset:?}");
                panic!("frame error");
            }
        }
    }
    let addr = l4_frame.start_address() + u64::from(addr.page_offset());
}
