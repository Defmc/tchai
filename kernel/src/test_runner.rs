use core::ptr::null_mut;

use alloc::{boxed::Box, vec::Vec};

use crate::{info, okay};

pub const TESTS: &[(&'static str, fn())] = &[
    ("equality", eq_assertion),
    ("floating point arithmetic", float_arithmetic),
    ("breakpoint exception", breakpoint_exception),
    (
        "kernel stackoverflow exception",
        kernel_stackoverflow_exception,
    ),
    // ("page fault exception", page_fault_exception),
    // ("memory access privileges", memory_access_privileges),
    ("memory allocation box", memory_allocation_box),
    ("memory allocation vec", memory_allocation_vectors),
];

pub fn run_tests() {
    executor(TESTS);
}

pub fn executor(tests: &[(&'static str, fn())]) {
    info!("running {} tests", tests.len());
    for (name, test) in tests.iter() {
        info!("\trunning test '{name}'");
        test();
        okay!("\ttest suceeded");
    }
}

pub fn eq_assertion() {
    assert_eq!(1, 1);
}

pub fn float_arithmetic() {
    assert_eq!(0.1f32 + 0.2f32, 0.3f32);
    assert_ne!(0.1f64 + 0.2f64, 0.3f64);
}

pub fn breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

pub fn kernel_stackoverflow_exception() {
    #[cfg(feature = "test_double_fault")]
    #[allow(unconditional_recursion)]
    fn overflow() {
        overflow();
        let x = 0;
        unsafe {
            core::ptr::write_volatile(x as *mut u8, 1);
        }
    }
    #[cfg(not(feature = "test_double_fault"))]
    fn overflow() {
        x86_64::instructions::nop()
    }
    overflow()
}

pub fn page_fault_exception() {
    unsafe {
        let x = core::ptr::read_volatile(0xdeadbeef as *const u8);
        assert_eq!(x, core::ptr::read_volatile(0xdeadbeef as *const u8))
    }
}

pub fn memory_access_privileges() {
    unsafe {
        let x = core::ptr::read_volatile(0x0 as *const u8);
        assert_eq!(x, core::ptr::read_volatile(0x0 as *const u8))
    }
}

pub fn memory_allocation_box() {
    let x = Box::new(0);
    assert_ne!(Box::into_raw(x), null_mut() as *mut i32);
}

pub fn memory_allocation_vectors() {
    let v: Vec<u8> = (0..100).collect();
    assert_eq!(v.len(), 100);
}
