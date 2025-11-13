use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello from Rust, {}! The cosmos awaits.", name)
}

#[wasm_bindgen]
pub fn distance_earth_moon_km() -> f64 {
    384400.0 // ave in kms
}
