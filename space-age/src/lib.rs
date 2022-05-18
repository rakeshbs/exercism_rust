// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_SECONDS: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration {
    d: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { d: s as f64 }
    }
}

pub trait Planet {
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

macro_rules! impl_Planet {
    ($t:ty,$a:expr) => {
        impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                d.d / ($a * EARTH_SECONDS)
            }
        }
    };
}

impl_Planet!(Mercury, 0.2408467_f64);
impl_Planet!(Venus, 0.61519726);
impl_Planet!(Earth, 1.0);
impl_Planet!(Mars, 1.8808158);
impl_Planet!(Jupiter, 11.862615);
impl_Planet!(Saturn, 29.447498);
impl_Planet!(Uranus, 84.016846);
impl_Planet!(Neptune, 164.79132);
