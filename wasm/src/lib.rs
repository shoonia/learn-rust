#[unsafe(no_mangle)]
pub extern "C" fn squarer(a: f64) -> f64 {
    a * a
}
