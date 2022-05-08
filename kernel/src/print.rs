#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::bsp::_print(format_args!($($arg)*)));
}