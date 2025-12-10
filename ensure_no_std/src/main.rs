#![deny(warnings)]

#![no_std]
#![no_main]

use core::ffi::{c_char, c_int};
use core::panic::PanicInfo;
use panic_no_std::panic;

#[cfg(windows)]
#[link(name="msvcrt")]
extern "C" { }

#[panic_handler]
fn panic_hanlder(info: &PanicInfo) -> ! {
    panic(info, 0)
}

#[unsafe(no_mangle)]
extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    panic!("PANIC");
}
