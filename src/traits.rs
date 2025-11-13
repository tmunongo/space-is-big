use crate::models::Distance;

pub trait DisplayDistance {
    fn format_distance(&self) -> String;
}

impl DisplayDistance for Distance {
    fn format_distance(&self) -> String {
        let km = self.to_km();

        if km < 1_000_000.0 {
            format!("{:.0} km", km)
        } else if km < 149_597_870.0 * 10.0 {
            format!("{:.3} AU", self.to_au())
        } else {
            format!("{:.2} light-minutes", self.to_light_minutes())
        }
    }
}