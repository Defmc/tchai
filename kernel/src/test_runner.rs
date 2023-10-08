use crate::{info, okay, println};

pub const TESTS: &[(&'static str, fn())] = &[
    ("equality", eq_assertion),
    ("floating point arithmetic", float_arithmetic),
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
