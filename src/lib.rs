#![feature(panic_info_message)]

#![deny(warnings)]

#![no_std]

use core::fmt::{self};
use core::fmt::Write as fmt_Write;
use core::panic::PanicInfo;
use exit_no_std::exit;

#[cfg(all(not(target_os="dos"), windows))]
fn write_ascii_char(c: u8) {
    use core::ptr::null_mut;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::fileapi::WriteFile;
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_ERROR_HANDLE;

    let stderr = unsafe { GetStdHandle(STD_ERROR_HANDLE) };
    if stderr != null_mut() && stderr != INVALID_HANDLE_VALUE {
        let mut written: DWORD = 0;
        unsafe { WriteFile(stderr, &c as *const _ as _, 1, &mut written as *mut _, null_mut()); }
    }
}

#[cfg(all(not(target_os="dos"), not(windows)))]
fn write_ascii_char(c: u8) {
    unsafe { libc::write(2, &c as *const _ as _, 1); }
}

#[cfg(target_os="dos")]
fn write_ascii_char(c: u8) {
    use pc_ints::int_21h_ah_02h_out_ch;

    if c == b'\n' {
        int_21h_ah_02h_out_ch(b'\r');
    }
    int_21h_ah_02h_out_ch(c);
}

struct LastChanceWriter;

impl fmt::Write for LastChanceWriter {
    fn write_char(&mut self, c: char) -> fmt::Result {
        let c = c as u32;
        let c = if c > 0x7F || c == '\r' as u32 {
            b'?'
        } else {
            c as u8
        };
        write_ascii_char(c);
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }
}

pub fn panic(info: &PanicInfo, exit_code: u8) -> ! {
    let _ = LastChanceWriter.write_str("Panic");
    if let Some(&message) = info.message() {
        let _ = write!(LastChanceWriter, ": {message}");
    } else if let Some(message) = info.payload().downcast_ref::<&str>() {
        let _ = write!(LastChanceWriter, ": {message}");
    }
    if let Some(location) = info.location() {
        let _ = write!(LastChanceWriter, " ({location})");
    }
    let _ = writeln!(LastChanceWriter, ".");
    exit(exit_code)
}
