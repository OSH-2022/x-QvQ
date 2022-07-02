mod auxiliary;
mod core_timer;

pub use auxiliary::MINI_UART;
pub use core_timer::{CORE_TIMER, CoreTimer};

use core::{marker::PhantomData, ops};

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
