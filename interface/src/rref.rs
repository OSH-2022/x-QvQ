use core::ops::{Deref, DerefMut};

pub unsafe auto trait RRefable {}
impl<T> !RRefable for *mut T {}
impl<T> !RRefable for *const T {}
impl<T> !RRefable for &T {}
impl<T> !RRefable for &mut T {}
impl<T> !RRefable for [T] {}

pub struct RRef<T>
where
    T: 'static + RRefable,
{
    domain_id_pointer: *mut usize,
    borrow_count_pointer: *mut usize,
    value_pointer: *mut T,
}

impl<T: RRefable> RRef<T> {
    pub fn borrow(&self) {
        unsafe {
            *self.borrow_count_pointer += 1;
        }
    }

    pub fn forfeit(&self) {
        unsafe {
            assert_ne!(*self.borrow_count_pointer, 0);
            *self.borrow_count_pointer -= 1;
        }
    }

    pub fn borrow_count(&self) -> usize {
        unsafe { *self.borrow_count_pointer }
    }

    pub fn move_to(&self, new_domain_id: usize) {
        unsafe { *self.domain_id_pointer = new_domain_id };
    }

    // Super unsafe from an ownership perspective
    pub(crate) unsafe fn ptr_mut(&self) -> &mut T {
        unsafe { &mut *self.value_pointer }
    }

    pub(crate) fn domain_id(&self) -> usize {
        unsafe { *self.domain_id_pointer }
    }
}

impl<T: RRefable> Drop for RRef<T> {
    fn drop(&mut self) {
        //self.cleanup();
    }
}

impl<T: RRefable> Deref for RRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.value_pointer }
    }
}

impl<T: RRefable> DerefMut for RRef<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.value_pointer }
    }
}
