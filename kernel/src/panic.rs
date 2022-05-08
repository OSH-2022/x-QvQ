use crate::print;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    if let Some(l) = info.location() {
        let _ = print!("panicked at {}:{}\n", l.file(), l.line());
    }
    if let Some(s) = info.message() {
        let _ = print!("{}\n", s);
    }
    loop {}
}
