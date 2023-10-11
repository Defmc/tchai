#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader_api::config::Mapping;
use bootloader_api::BootloaderConfig;
use kernel::monitor::RgbColor;
use kernel::{okay, print, println};

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    #[cfg(not(feature = "test_double_fault"))]
    kernel::log!(kernel::ERRO_COLOR, "CRITICAL ERRO", "panicked");
    #[cfg(feature = "test_double_fault")]
    kernel::okay!("\ttest succeed");

    kernel::ints::idle_mode();
}

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

bootloader_api::entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

#[no_mangle]
fn kernel_main(info: &'static mut bootloader_api::BootInfo) -> ! {
    kernel::init(info);

    #[cfg(debug_assertions)]
    kernel::test_runner::run_tests();

    print!("yaay, welcome to ");
    println!(RgbColor::new(0, 255, 0) => "tchaiOS!");

    print!("(root) [/]: ");

    loop {
        let ticks = kernel::ints::get_ticks();
        while kernel::ints::get_ticks() - ticks < 20 {}
        okay!("still running");
        #[cfg(test)]
        info!("test mode");
    }
}
