use crate::framebuffer::WRITER;
use core::{fmt, ptr};

#[macro_export]
#[allow(clippy::unseparated_literal_suffix)]
macro_rules! print {
    ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = WRITER.lock();
    writer.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! set_color {
    ($color:expr) => {
        $crate::framebuffer::WRITER.lock().set_color($color);
    };
}
