use libc::{c_char, c_void};
use std::ptr::{null, null_mut};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

extern "C" fn write_cb(_: *mut c_void, message: *const c_char) {
    print!("{}", String::from_utf8_lossy(unsafe {
        std::ffi::CStr::from_ptr(message as *const i8).to_bytes()
    }));
}

fn mem_print() {
    unsafe { jemalloc_sys::malloc_stats_print(Some(write_cb), null_mut(), null()) }
}

fn test_closure<F>(f: F)
where F: Fn()
{
    mem_print();
    f();
    mem_print();
}

#[cfg(feature = "mem_use")]
fn main() {
    test_closure(|| {
        let _heap = Vec::<u8>::with_capacity (1024 * 128);
    });
}

#[cfg(not(feature = "mem_use"))]
fn main() {
}