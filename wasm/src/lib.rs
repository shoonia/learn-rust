use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn squarer(a: f64) -> f64 {
    a * a
}
