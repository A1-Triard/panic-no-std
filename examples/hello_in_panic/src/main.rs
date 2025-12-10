#![feature(extern_types)]

#![deny(warnings)]

#![windows_subsystem="console"]
#![no_std]
#![no_main]

#[cfg(dos)]
extern crate rlibc_ext;

mod no_std {
    #[panic_handler]
    fn panic_handler(info: &core::panic::PanicInfo) -> ! { panic_no_std::panic(info, b'P') }
}

#[cfg(not(dos))]
use core::ffi::{c_char, c_int};

#[cfg(not(dos))]
#[unsafe(no_mangle)]
extern "C" fn main(_: c_int, _: *mut *mut c_char) -> c_int {
    start();
    0
}

#[cfg(dos)]
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn mainCRTStartup() -> ! {
    start();
    exit_no_std::exit(0)
}

fn start() {
    panic!("Hello! I am panicking");
}
