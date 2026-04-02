use std::hint::black_box;

#[inline(never)]
pub fn copy_fn<T: Copy>(a: T) -> T {
    black_box(a)
}

#[unsafe(no_mangle)]
pub fn add_one(num: i32) -> i32 {
    num
}

fn main() {
    println!("nothing to see run one of the binaries instead, simply testing looking at assembly");
    let _ = copy_fn((1,2,3, "test"));
}
