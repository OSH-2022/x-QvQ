mod auxiliary;

pub use super::memory::map;
pub use auxiliary::MINI_UART;

use core::{marker::PhantomData, ops};

trait Driver {
    fn init(&self);
}

struct RegisterWrapper<T> {
    start: usize,
    phantom: PhantomData<T>,
}

impl<T> RegisterWrapper<T> {
    const fn new(start: usize) -> Self {
        Self {
            start,
            phantom: PhantomData,
        }
    }
}

impl<T> ops::Deref for RegisterWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.start as *const _) }
    }
}

pub fn driver_init() {
    {
        let mini_uart = MINI_UART.lock();
        mini_uart.init();
    }
}
