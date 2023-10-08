#![no_std]
#![no_main]

use bootloader_api::info::FrameBuffer;
use monitor::{FrameBufferWriter, RgbColor};

pub mod monitor;
#[cfg(debug_assertions)]
pub mod test_runner;

pub static mut MONITOR_OUT: Option<FrameBufferWriter> = None;

pub fn setup_monitor(fb: &'static mut FrameBuffer) {
    let fb_info = fb.info();
    let fb_buffer = fb.buffer_mut();
    let monitor = monitor::FrameBufferWriter::new(fb_buffer, fb_info);
    unsafe {
        MONITOR_OUT = Some(monitor);
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
        use core::fmt::Write;
        unsafe { write!($crate::MONITOR_OUT.as_mut().unwrap(), $($args),*).unwrap() }
    }};
    ($color:expr => $($args:expr),*) => {{
        let formatted = format_args!($($args),*);
        unsafe { $crate::MONITOR_OUT.as_mut().unwrap().write_colored_str(formatted.as_str().unwrap(), &$color) }
    }}
}

#[macro_export]
macro_rules! println {
    ($($args:expr),*) => {{
        use core::fmt::Write;
        unsafe { writeln!($crate::MONITOR_OUT.as_mut().unwrap(), $($args),*).unwrap() }
    }};
    ($color:expr => $($args:expr),*) => {{
        use core::fmt::Write;
        let formatted = format_args!($($args),*);
        unsafe { $crate::MONITOR_OUT.as_mut().unwrap().write_colored_str(formatted.as_str().unwrap(), &$color) }
        println!("");
    }}
}
