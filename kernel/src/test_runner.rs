use crate::{info, okay, println};

pub const TESTS: &[(&'static str, fn())] = &[
    ("equality", eq_assertion),
    ("floating point arithmetic", float_arithmetic),
    ("breakpoint exception", breakpoint_exception),
    // (
    //     "kernel stackoverflow exception",
    //     kernel_stackoverflow_exception,
    // ),
];

pub fn run_tests() {
    executor(TESTS);
}

pub fn executor(tests: &[(&'static str, fn())]) {
    info!("running {} tests", tests.len());
    for (name, test) in tests.iter() {
        info!("running test '{name}'");
        test();
        okay!("test suceeded");
        println!("");
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
    // #[allow(unconditional_recursion)]
    // fn overflow() {
    //     overflow();
    //     let x = 0;
    //     unsafe {
    //         core::ptr::write_volatile(x as *mut u8, 1);
    //     }
    // }
    // overflow()
}
