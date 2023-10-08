#![no_std]
#![no_main]

use core::panic::PanicInfo;

use kernel::monitor::RgbColor;
use kernel::{print, println};

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    kernel::log!(kernel::ERRO_COLOR, "CRITICAL ERRO", "{info:#?}");
    loop {}
}

bootloader_api::entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(info: &'static mut bootloader_api::BootInfo) -> ! {
    kernel::setup_monitor(info.framebuffer.as_mut().unwrap());

    #[cfg(debug_assertions)]
    kernel::test_runner::run_tests();

    print!("yaay, welcome to ");
    println!(RgbColor::new(0, 255, 0) => "tchaiOS");

    println!("(root) [/]: ");
    panic!("to be continued...");
}
