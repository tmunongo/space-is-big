use wasm_bindgen::prelude::*;

use crate::models::Distance;
use crate::solar_system;

#[wasm_bindgen]
pub fn get_all_bodies() -> Result<JsValue, JsValue> {
    let bodies = solar_system::all_bodies();
    serde_wasm_bindgen::to_value(&bodies)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn distance_from_earth(planet_name: &str) -> Result<f64, JsValue> {
    let earth = solar_system::earth();
    let bodies = solar_system::all_bodies();
    let target = bodies
        .iter()
        .find(|b| b.name.eq_ignore_ascii_case(planet_name))
        .ok_or_else(|| {
            JsValue::from_str(&format!(
                "Planet '{}' 
        not found",
                planet_name
            ))
        })?;

    let distance = earth.distance_to(target);

    Ok(distance.to_km())
}

#[wasm_bindgen]
pub fn km_to_au(km: f64) -> f64 {
    Distance::Kilometers(km).to_au()
}

#[wasm_bindgen]
pub fn get_planet_info() -> JsValue {
    use crate::solar_system;

    let earth = solar_system::earth();

    serde_wasm_bindgen::to_value(&earth).unwrap()
}

#[wasm_bindgen]
pub fn au_to_light_minutes(au: f64) -> f64 {
    Distance::AstronomicalUnits(au).to_light_minutes()
}

#[wasm_bindgen]
pub fn compare_distances(planet1: &str, planet2: &str) -> Result<f64, JsValue> {
    let bodies = solar_system::all_bodies();
    let body1 = bodies
        .iter()
        .find(|b| b.name.eq_ignore_ascii_case(planet1))
        .ok_or_else(|| JsValue::from_str(&format!("Planet '{}' not found", planet1)))?;
    let body2 = bodies
        .iter()
        .find(|b| b.name.eq_ignore_ascii_case(planet2))
        .ok_or_else(|| JsValue::from_str(&format!("Planet '{}' not found", planet2)))?;
    let dist1 = body1.distance_from_sun.to_km();
    let dist2 = body2.distance_from_sun.to_km();
    if dist1 == 0.0 {
        return Err(JsValue::from_str("Cannot divide by zero"));
    }
    Ok(dist2 / dist1)
}
