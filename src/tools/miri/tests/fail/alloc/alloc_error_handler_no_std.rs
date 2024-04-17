//@compile-flags: -Cpanic=abort
#![feature(start, core_intrinsics)]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]
#![no_std]

extern crate alloc;

use alloc::alloc::*;

extern "Rust" {
    fn miri_write_to_stderr(bytes: &[u8]);
}

// The default no_std alloc_error_handler is a panic.

#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    let msg = "custom panic handler called!\n";
    unsafe { miri_write_to_stderr(msg.as_bytes()) };
    core::intrinsics::abort(); //~ERROR: aborted
}

// rustc requires us to provide some more things that aren't actually used by this test
mod plumbing {
    use super::*;

    struct NoAlloc;

    unsafe impl GlobalAlloc for NoAlloc {
        unsafe fn alloc(&self, _: Layout) -> *mut u8 {
            unreachable!();
        }

        unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
            unreachable!();
        }
    }

    #[global_allocator]
    static GLOBAL: NoAlloc = NoAlloc;
}

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    handle_alloc_error(Layout::for_value(&0));
}
