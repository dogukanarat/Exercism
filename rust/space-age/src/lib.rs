// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration
{
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration{ seconds: s }
    }
}

pub enum PlanetCoeff {
    Mercury = 2408467,
    Venus   = 6151972,
    Earth   = 10000000,
    Mars    = 18808158,
    Jupiter = 118626150,
    Saturn  = 294474980,
    Uranus  = 840168460,
    Neptune = 1647913200,
}

pub trait Planet {
    fn fixed_point_conversion(seconds: PlanetCoeff) -> f64 {
        seconds as i64 as f64 / 10_000_000.0
    }

    fn years_in_earth(seconds: u64) -> f64 {
        seconds as f64 / (60.0 * 60.0 * 24.0 * 365.25)
    }

    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({:?}) to the number of years on this planet for that duration",
            d,
        );
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
       Self::years_in_earth(d.seconds) / Self::fixed_point_conversion(PlanetCoeff::Mercury)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
         Self::years_in_earth(d.seconds) / Self::fixed_point_conversion(PlanetCoeff::Venus)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        Self::years_in_earth(d.seconds) / Self::fixed_point_conversion(PlanetCoeff::Earth)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        Self::years_in_earth(d.seconds) / Self::fixed_point_conversion(PlanetCoeff::Mars)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        Self::years_in_earth(d.seconds) / Self::fixed_point_conversion(PlanetCoeff::Jupiter)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        Self::years_in_earth(d.seconds) / Self::fixed_point_conversion(PlanetCoeff::Saturn)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        Self::years_in_earth(d.seconds) / Self::fixed_point_conversion(PlanetCoeff::Uranus)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        Self::years_in_earth(d.seconds) / Self::fixed_point_conversion(PlanetCoeff::Neptune)
    }
}
