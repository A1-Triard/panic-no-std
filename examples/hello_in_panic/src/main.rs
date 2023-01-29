#![feature(extern_types)]
#![feature(start)]

#![deny(warnings)]

#![windows_subsystem="console"]
#![no_std]
#![cfg_attr(target_os="dos", no_main)]

extern crate pc_atomics;
extern crate rlibc;

mod no_std {
    #[no_mangle]
    extern "C" fn _aulldiv() -> ! { panic!("10") }
    #[no_mangle]
    extern "C" fn _aullrem() -> ! { panic!("11") }
    #[no_mangle]
    extern "C" fn _chkstk() { }
    #[no_mangle]
    extern "C" fn _fltused() -> ! { panic!("13") }
    #[no_mangle]
    extern "C" fn strlen() -> ! { panic!("14") }

    #[panic_handler]
    fn panic_handler(info: &core::panic::PanicInfo) -> ! { panic_no_std::panic(info, b'P') }
}

#[cfg(target_os="dos")]
extern {
    type PEB;
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
extern "stdcall" fn mainCRTStartup(_: *const PEB) -> u64 {
    start();
    0
}

fn start() {
    panic!("Hello! I am panicking.");
}