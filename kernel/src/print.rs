use crate::bsp::MINI_UART;
use core::fmt::{Arguments, Result};

pub fn _print(args: Arguments) -> Result {
    MINI_UART.lock_and_write(args)
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}