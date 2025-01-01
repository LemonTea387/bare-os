use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

/// Standard first serial interface port
const SERIAL_PORT_1: u16 = 0x3F8;

lazy_static! {
    pub static ref Serial1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(SERIAL_PORT_1) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    Serial1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed")
}
