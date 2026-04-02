#![allow(unsafe_op_in_unsafe_fn)]

use std::alloc::{dealloc, Layout};

#[inline]
unsafe fn cycle_counter() -> u64 {
    let value: u64;
    unsafe {
        core::arch::asm!(
        "isb",
        "mrs {val}, cntvct_el0",
        val = out(reg) value,
        options(nostack, preserves_flags)
        );
    }
    value
}

#[inline]
pub fn mfence() {
    unsafe {
        core::arch::asm!(
        "dmb ish",
        options(nostack, preserves_flags)
        );
    }
}

#[inline]
unsafe fn flush_pointer(ptr: *const u8) {
    core::arch::asm!(
    "dc civac, {addr}",
    addr = in(reg) ptr,
    options(nostack, preserves_flags)
    );


    core::arch::asm!("dsb ish", options(nostack, preserves_flags));
}

unsafe fn cache_line() {
    let layout = Layout::from_size_align(4096, 4096).unwrap();
    let ptr = std::alloc::alloc_zeroed(layout);
    flush_pointer(ptr);
    if ptr.is_null() {
        panic!("Something went very wrong")
    }

    let attempts = 1024;
    for offset in 0..1024 {
        let mut results = Vec::with_capacity(attempts);
        for _ in 0..attempts {
            flush_pointer(ptr);
            flush_pointer(ptr.add(offset));
            mfence();
            std::ptr::read_volatile(ptr);
            mfence();

            let t0 = cycle_counter();
            std::ptr::read_volatile(ptr.add(offset));
            let t1 = cycle_counter();
            results.push(t1 - t0);
        }
    }


    dealloc(ptr, layout);
}

fn main() {
    unsafe { cache_line() }
}
