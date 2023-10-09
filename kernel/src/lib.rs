#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::fmt;

use bootloader_api::info::FrameBuffer;
use monitor::{FrameBufferWriter, RgbColor};

pub mod gdt;
pub mod ints;
pub mod monitor;
#[cfg(debug_assertions)]
pub mod test_runner;

use spin::mutex::Mutex;
pub static mut MONITOR_OUT: Mutex<Option<FrameBufferWriter>> = Mutex::new(None);

pub fn init() {
    ints::init_idt();
    gdt::init_gdt();
    unsafe { ints::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn setup_monitor(fb: &'static mut FrameBuffer) {
    let fb_info = fb.info();
    let fb_buffer = fb.buffer_mut();
    let monitor = monitor::FrameBufferWriter::new(fb_buffer, fb_info);
    unsafe {
        MONITOR_OUT = Some(monitor).into();
    }
}

pub const INFO_COLOR: RgbColor = RgbColor::new(127, 127, 127);
pub const OKAY_COLOR: RgbColor = RgbColor::new(0, 255, 0);
pub const WARN_COLOR: RgbColor = RgbColor::new(255, 255, 0);
pub const ERRO_COLOR: RgbColor = RgbColor::new(255, 0, 0);
pub const WHITE_COLOR: RgbColor = RgbColor::new(255, 255, 255);

#[macro_export]
macro_rules! log {
    ($color:expr, $msg:expr, $($args:expr),*) => {{
        $crate::print!($crate::WHITE_COLOR => "[");
        $crate::print!($color => $msg);
        $crate::print!($crate::WHITE_COLOR => "] ");
        $crate::println!($($args),*);
    }};
}

#[macro_export]
macro_rules! info {
    ($($args:expr),*) => {
        $crate::log!($crate::INFO_COLOR, "info", $($args),*)
    };
}

#[macro_export]
macro_rules! okay {
    ($($args:expr),*) => {
        $crate::log!($crate::OKAY_COLOR, "okay", $($args),*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:expr),*) => {
        $crate::log!($crate::WARN_COLOR, "warn", $($args),*)
    };
}

#[macro_export]
macro_rules! erro {
    ($($args:expr),*) => {
        $crate::log!($crate::ERRO_COLOR, "erro", $($args),*)
    };
}

#[macro_export]
macro_rules! print {
    ($($args:expr),*) => {{
        $crate::println!($crate::monitor::RgbColor::new(255, 255, 255) => $($args),*);
    }};
    ($color:expr => $($args:expr),*) => {{
        $crate::internal_colored_print(format_args!($($args),*), &$color);
    }}
}

#[macro_export]
macro_rules! println {
    ($($args:expr),*) => {{
        $crate::println!($crate::monitor::RgbColor::new(255, 255, 255) => $($args),*);
    }};
    ($color:expr => $($args:expr),*) => {{
        $crate::internal_colored_print(format_args!($($args),*), &$color);
        $crate::internal_colored_print(format_args!("\n"), &$color);
    }}
}

pub fn internal_colored_print(fmt: fmt::Arguments, color: &RgbColor) {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| unsafe {
        crate::MONITOR_OUT
            .lock()
            .as_mut()
            .unwrap()
            .write_colored_str(fmt.as_str().unwrap(), color);
    })
}
