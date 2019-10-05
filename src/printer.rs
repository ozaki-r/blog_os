use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

pub struct SerialPort {
    addr: u32,
}

impl SerialPort {
    pub const unsafe fn new(addr: u32) -> SerialPort {
        SerialPort {
            addr,
        }
    }

    pub fn write_string(&mut self, s: &str) {
        let addr = self.addr as *mut u8;
        // XXX s.bytes() doesn't work
        let s = s.as_bytes();
        for c in s.iter() {
            unsafe {
                *addr = *c as u8;
            }
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    /// A global `Writer` instance that can be used for printing to the VGA text buffer.
    ///
    /// Used by the `print!` and `println!` macros.
    pub static ref WRITER: Mutex<SerialPort> = {
       let serial_port = unsafe { SerialPort::new(0x1001_3000) };
       Mutex::new(serial_port)
    };
}

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::printer::_print(format_args!($($arg)*)));
}

/// Like the `println!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints the given formatted string to the VGA text buffer through the global `WRITER` instance.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
