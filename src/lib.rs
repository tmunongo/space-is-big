use wasm_bindgen::prelude::*;

pub mod models;
pub mod solar_system;
pub mod traits;
pub mod wasm_exports;

pub use wasm_exports::*;

pub use models::{CelestialBody, CelestialBodyType, Distance};

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello from Rust, {}! The cosmos awaits.", name)
}

#[wasm_bindgen]
pub fn distance_earth_moon_km() -> f64 {
    384400.0 // ave in kms
}
