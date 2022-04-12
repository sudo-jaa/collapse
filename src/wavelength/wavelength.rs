#[allow(dead_code)]
use uom::si::f64::{Frequency, Length};

#[derive(Debug, Copy, Clone)]
pub struct Wavelength {
    pub peak_frequency: Frequency,
    pub peak_wavelength: Length,
}

impl Wavelength {
    pub fn new(wavelength: Length, frequency: Frequency) -> Wavelength {
        Wavelength {
            peak_wavelength: wavelength,
            peak_frequency: frequency,
        }
    }
}
