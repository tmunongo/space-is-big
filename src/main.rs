use interspace::solar_system;
use interspace::traits::DisplayDistance;

fn main() {
    println!("🌌 Interspace CLI - Solar System Distance Explorer");

    println!("{}", "=".repeat(60));

    println!();

    let earth = solar_system::earth();
    let mars = solar_system::all_bodies()
        .into_iter()
        .find(|body| body.name == "Mars")
        .expect("Mars not found");

    let earth_mars = earth.distance_to(&mars);
    println!("\n  Distance between Earth and Mars:");
    println!("    {:.2e} km", earth_mars.to_km());
    println!("    {:.3} AU", earth_mars.to_au());

    println!(
        "Mars distance from Sun: {}",
        mars.distance_from_sun.format_distance()
    );
    println!(
        "Earth distance from Sun: {}",
        earth.distance_from_sun.format_distance()
    );
}
