#![allow(path_statements, unsafe_op_in_unsafe_fn)]
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Instant;

const PADDING_LEN: usize = 120;

#[repr(C, align(256))]
struct Probe {
    foo: u64,
    _padding: [u8; PADDING_LEN],
    bar: u64,
}

#[derive(Copy, Clone)]
struct Container {
    inner: *mut Probe,
}

unsafe impl Sync for Container {}
unsafe impl Send for Container {}

unsafe fn false_sharing() {
    let done: AtomicBool = AtomicBool::new(false);
    let container = Container {
        inner: &mut Probe {
            foo: 0,
            _padding: [0; PADDING_LEN],
            bar: 0,
        },
    };

    thread::scope(|s| {
        s.spawn(|| {
            container;
            while !done.load(Relaxed) {
                std::ptr::write_volatile(&mut (*container.inner).foo, (*container.inner).foo + 1);
            }
        });

        s.spawn(|| {
            container;
            for _ in 0..1_000_000_000 {
                std::ptr::read_volatile(&(*container.inner).bar);
            }
            done.store(true, Relaxed);
        });
    });
    println!("{}", (*container.inner).foo);
}

fn main() {
    let start = Instant::now();
    unsafe {
        false_sharing();
    }

    println!("completed in {}ms", start.elapsed().as_millis())
}
