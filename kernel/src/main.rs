#![no_std]
#![no_main]

use core::panic::PanicInfo;

use kernel::monitor::RgbColor;
use kernel::{okay, print, println};

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    kernel::log!(kernel::ERRO_COLOR, "CRITICAL ERRO", "panicked");
    loop {}
}

bootloader_api::entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(info: &'static mut bootloader_api::BootInfo) -> ! {
    kernel::setup_monitor(info.framebuffer.as_mut().unwrap());
    kernel::init();

    #[cfg(debug_assertions)]
    kernel::test_runner::run_tests();

    print!("yaay, welcome to ");
    println!(RgbColor::new(0, 255, 0) => "tchaiOS");

    println!("(root) [/]: ");

    loop {
        for _ in 0..100_000_00 {
            let x = 0;
            unsafe {
                assert_eq!(
                    core::ptr::read_volatile(&x as *const i32),
                    core::ptr::read_volatile(&x as *const i32)
                )
            }
        }
        okay!("still running. {} ticks", unsafe {
            kernel::ints::TIMER_TICKS
        });
    }
}
