#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod monitor;
use core::fmt::Write;
use monitor::FrameBufferWriter;

use crate::monitor::RgbColor;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(info: &'static mut bootloader_api::BootInfo) -> ! {
    let fb = info.framebuffer.as_mut().unwrap();
    let fb_info = fb.info();
    let mut monitor = FrameBufferWriter::new(fb.buffer_mut(), fb_info);
    write!(monitor, "baay, welcome to ").unwrap();
    monitor.write_colored_str("tchaiOS", &RgbColor::new(0, 255, 0));
    writeln!(monitor, "\n(root) [/]: ").unwrap();
    loop {}
}
