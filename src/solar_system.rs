use crate::models::{CelestialBody, CelestialBodyType, Distance};

pub fn sun() -> CelestialBody {
    CelestialBody::new(
        "Sun",
        CelestialBodyType::Star,
        696_000.0,
        Distance::Kilometers(0.0),
    )
}

pub fn earth() -> CelestialBody {
    CelestialBody::new(
        "Earth",
        CelestialBodyType::Planet,
        6_371.0,
        Distance::AstronomicalUnits(1.0),
    )
}

pub fn moon() -> CelestialBody {
    CelestialBody::new(
        "Moon",
        CelestialBodyType::Moon,
        1_737.4,
        Distance::AstronomicalUnits(1.0),
    )
}

pub fn all_bodies() -> Vec<CelestialBody> {
    vec![
        sun(),
        CelestialBody::new(
            "Mercury",
            CelestialBodyType::Planet,
            2_439.7,
            Distance::AstronomicalUnits(0.39),
        ),
        CelestialBody::new(
            "Venus",
            CelestialBodyType::Planet,
            6_051.8,
            Distance::AstronomicalUnits(0.72),
        ),
        earth(),
        CelestialBody::new(
            "Mars",
            CelestialBodyType::Planet,
            3_389.5,
            Distance::AstronomicalUnits(1.52),
        ),
        CelestialBody::new(
            "Jupiter",
            CelestialBodyType::Planet,
            69_911.0,
            Distance::AstronomicalUnits(5.20),
        ),
        CelestialBody::new(
            "Saturn",
            CelestialBodyType::Planet,
            58_232.0,
            Distance::AstronomicalUnits(9.54),
        ),
        CelestialBody::new(
            "Uranus",
            CelestialBodyType::Planet,
            25_362.0,
            Distance::AstronomicalUnits(19.19),
        ),
        CelestialBody::new(
            "Neptune",
            CelestialBodyType::Planet,
            24_622.0,
            Distance::AstronomicalUnits(30.07),
        ),
    ]
}
