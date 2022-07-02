use crate::bsp::MINI_UART;
use core::fmt::{Arguments, Result, Write};

pub fn _print(args: Arguments) -> Result {
    let mut uart = MINI_UART.lock();
    uart.write_fmt(args)
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}