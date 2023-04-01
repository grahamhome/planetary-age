mod tests;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    const SEC_PER_YEAR: f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / Self::SEC_PER_YEAR
    }
}

const SECONDS_PER_EARTH_YEAR: f64 = 31557600.0;

macro_rules! planet {
    ($n:ident, $p:expr) => {
        pub struct $n; impl Planet for $n { const SEC_PER_YEAR: f64 = $p; }
    }
}

planet!(Mercury, 0.2408467 * SECONDS_PER_EARTH_YEAR);
planet!(Venus, 0.61519726 * SECONDS_PER_EARTH_YEAR);
planet!(Earth, SECONDS_PER_EARTH_YEAR);
planet!(Mars, 1.8808158 * SECONDS_PER_EARTH_YEAR);
planet!(Jupiter, 11.862615 * SECONDS_PER_EARTH_YEAR);
planet!(Saturn, 29.447498 * SECONDS_PER_EARTH_YEAR);
planet!(Uranus, 84.016846 * SECONDS_PER_EARTH_YEAR);
planet!(Neptune, 164.79132 * SECONDS_PER_EARTH_YEAR);

