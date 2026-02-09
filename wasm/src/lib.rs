use std::mem::forget;
use std::ptr::copy as ptr_copy;
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

#[unsafe(no_mangle)]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    forget(buf);
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    unsafe {
        Vec::from_raw_parts(ptr, len, len);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn to_uppercase(ptr: *const u8, len: usize, out_len_ptr: *mut usize) -> *mut u8 {
    let slice = unsafe { from_raw_parts(ptr, len) };
    let result = String::from_utf8_lossy(slice).to_uppercase();
    let bytes = result.into_bytes();
    let result_len = bytes.len();
    let result_ptr = alloc(result_len);
    unsafe {
        ptr_copy(bytes.as_ptr(), result_ptr, result_len);
        *out_len_ptr = result_len; // Write length to output pointer
    };
    forget(bytes);
    result_ptr
}
