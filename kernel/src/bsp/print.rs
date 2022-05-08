use super::driver::MINI_UART;
use core::fmt::{Arguments, Result, Write};

pub fn _print(args: Arguments) -> Result {
    let mut mini_uart = MINI_UART.lock();
    mini_uart.write_fmt(args)
}
