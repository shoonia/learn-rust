use std::slice::from_raw_parts;

#[unsafe(no_mangle)]
pub extern "C" fn squarer(a: f64) -> f64 {
    a * a
}

#[unsafe(no_mangle)]
pub extern "C" fn sum(n: *const f64, len: usize) -> f64 {
    let numbers = unsafe { from_raw_parts(n, len) };
    numbers.iter().sum()
}
