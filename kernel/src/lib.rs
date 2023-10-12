#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use core::fmt;

use bootloader_api::{info::FrameBuffer, BootInfo};
use mem::BootInfoFrameAllocator;
use monitor::{FrameBufferWriter, RgbColor};

pub mod allocator;
pub mod gdt;
pub mod ints;
pub mod mem;
pub mod monitor;
#[cfg(debug_assertions)]
pub mod test_runner;

use spin::mutex::Mutex;
use uart_16550::SerialPort;
use x86_64::{
    structures::paging::{OffsetPageTable, Size4KiB},
    VirtAddr,
};
pub static mut MONITOR_OUT: Mutex<Option<FrameBufferWriter>> = Mutex::new(None);
pub static mut SERIAL_OUT: Mutex<Option<SerialPort>> = Mutex::new(None);

pub static mut PAGE_MAPPER: Option<OffsetPageTable> = None;
pub static mut FRAME_ALLOCATOR: Option<BootInfoFrameAllocator> = None;

pub const SERIAL_IO_PORT: u16 = 0x3F8;

pub fn init(info: &'static mut BootInfo) {
    setup_monitor(info.framebuffer.as_mut().unwrap());
    okay!("monitor started");

    gdt::init();
    ints::init();

    info!("initializing pics");
    unsafe { ints::PICS.lock().initialize() };
    okay!("initialized pics");

    info!("enabling interrupts");
    x86_64::instructions::interrupts::enable();
    okay!("enabled interrupts");

    info!("activing level 4 paging tables");
    unsafe {
        let addr = VirtAddr::new(*info.physical_memory_offset.as_ref().unwrap());
        PAGE_MAPPER = Some(mem::init(addr));
    }
    okay!("actived level 4 paging tables");

    info!("creating frame buffer allocator");
    unsafe { FRAME_ALLOCATOR = Some(mem::BootInfoFrameAllocator::new(&info.memory_regions)) };
    okay!("created frame buffer allocator");

    info!("paging heap");
    unsafe {
        allocator::init_heap(
            PAGE_MAPPER.as_mut().unwrap(),
            FRAME_ALLOCATOR.as_mut().unwrap(),
        )
        .unwrap();
    }
    okay!("paged heap");
}

pub fn setup_monitor(fb: &'static mut FrameBuffer) {
    let fb_info = fb.info();
    let fb_buffer = fb.buffer_mut();
    let monitor = monitor::FrameBufferWriter::new(fb_buffer, fb_info);
    unsafe {
        MONITOR_OUT = Some(monitor).into();
    }
    unsafe {
        let mut sp = SerialPort::new(SERIAL_IO_PORT);
        sp.init();
        SERIAL_OUT = Some(sp).into();
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
        $crate::print!($crate::monitor::RgbColor::new(255, 255, 255) => $($args),*);
    }};
    ($color:expr => $($args:expr),*) => {{
        $crate::internal_colored_print(format_args!($($args),*), $color);
    }}
}

#[macro_export]
macro_rules! println {
    ($($args:expr),*) => {{
        $crate::println!($crate::monitor::RgbColor::new(255, 255, 255) => $($args),*);
    }};
    ($color:expr => $($args:expr),*) => {{
        $crate::print!($color => $($args),*);
        $crate::print!("\n");
    }}
}

pub fn internal_colored_print(fmt: fmt::Arguments, color: RgbColor) {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        use core::fmt::Write;
        let mut serial_lock = unsafe { crate::SERIAL_OUT.lock() };
        let serial = serial_lock.as_mut().unwrap();
        serial.write_fmt(fmt).unwrap();

        let mut monitor_lock = unsafe { crate::MONITOR_OUT.lock() };
        let monitor = monitor_lock.as_mut().unwrap();
        monitor.color = color;
        monitor.write_fmt(fmt).unwrap();
    })
}
