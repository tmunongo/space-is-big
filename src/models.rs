#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Distance {
    Kilometers(f64),
    AstronomicalUnits(f64),
    LightMinutes(f64),
}

impl Distance {
    pub fn to_km(&self) -> f64 {
        match self {
            Distance::Kilometers(km) => *km,
            Distance::AstronomicalUnits(au) => au * 149_597_870.7,
            Distance::LightMinutes(lm) => lm * 17_987_547.48,
        }
    }

    pub fn to_au(&self) -> f64 {
        self.to_km() / 149_597_870.7
    }

    pub fn to_light_minutes(&self) -> f64 {
        self.to_km() / 17_987_547.48
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CelestialBodyType {
    Star,
    Planet,
    Moon,
    DwarfPlanet,
}

#[derive(Debug, Clone)]
pub struct CelestialBody {
    pub name: String,
    pub body_type: CelestialBodyType,
    pub radius_km: f64,
    pub distance_from_sun: Distance,
}

impl CelestialBody {
    pub fn new(
        name: &str,
        body_type: CelestialBodyType,
        radius_km: f64,
        distance_from_sun: Distance,
    ) -> Self {
        CelestialBody {
            name: name.to_string(),
            body_type,
            radius_km,
            distance_from_sun,
        }
    }

    pub fn distance_to(&self, other: &CelestialBody) -> Distance {
        let self_km = self.distance_from_sun.to_km();
        let other_km = other.distance_from_sun.to_km();
        Distance::Kilometers((self_km - other_km).abs())
    }
}
