use spin::{Lazy, RwLock};
use uart_16550::SerialPort;

pub static SERIAL1: Lazy<RwLock<SerialPort>> = Lazy::new(|| {
    RwLock::new({
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        serial_port
    })
});

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .write()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
