#![feature(extern_types)]
#![feature(start)]

#![deny(warnings)]

#![windows_subsystem="console"]
#![no_std]
#![cfg_attr(target_os="dos", no_main)]

extern crate rlibc_ext;

mod no_std {
    #[panic_handler]
    fn panic_handler(info: &core::panic::PanicInfo) -> ! { panic_no_std::panic(info, b'P') }
}

#[cfg(not(target_os="dos"))]
#[start]
fn main(_: isize, _: *const *const u8) -> isize {
    start();
    0
}

#[cfg(target_os="dos")]
#[allow(non_snake_case)]
#[no_mangle]
extern "C" fn mainCRTStartup() -> ! {
    start();
    exit_no_std::exit(0)
}

fn start() {
    panic!("Hello! I am panicking");
}
