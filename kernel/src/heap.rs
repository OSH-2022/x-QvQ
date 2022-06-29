use palloc::SpinPalloc;

#[global_allocator]
pub static mut ALLOCATOR: SpinPalloc = SpinPalloc::empty();