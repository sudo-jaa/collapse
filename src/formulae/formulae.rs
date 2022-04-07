#[allow(dead_code)]
use crate::formulae::constants::{
    BOLTZMANN_CONSTANT, GRAVITATIONAL_CONSTANT, STEFAN_BOLTZMANN_CONSTANT, ZERO_POINT_LUMINOSITY,
};
use crate::units::units::mass::solar_mass;
use crate::units::units::power::solar_luminosity;
use crate::units::units::time::million_year;
use colortemp::RGB;
use num::integer::cbrt;
use std::f64::consts;
use std::f64::consts::PI;
use uom::num_traits::real::Real;
use uom::num_traits::Pow;
use uom::si::f64::{
    Area, Length, Mass, MassDensity, Power, ThermodynamicTemperature, Time, Volume,
};
use uom::si::length::meter;
use uom::si::mass::{femtogram, kilogram};
use uom::si::power::watt;
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::time::year;
use uom::typenum;

pub mod moles {
    use crate::formulae::constants::GAS_CONSTANT;
    use uom::si::amount_of_substance::mole;
    use uom::si::f64::{AmountOfSubstance, Pressure, ThermodynamicTemperature, Volume};

    pub fn from_pressure_volume_temperature(
        pressure: Pressure,
        volume: Volume,
        temperature: ThermodynamicTemperature,
    ) -> AmountOfSubstance {
        AmountOfSubstance::new::<mole>(pressure.value * volume.value)
            / (GAS_CONSTANT * temperature.value)
    }
}

pub mod wavelength {
    use uom::fmt::DisplayStyle::Abbreviation;
    use uom::si::frequency::{centihertz, decihertz, gigahertz, hectohertz, kilohertz};
    use uom::si::length::centimeter;
    use uom::si::{
        f64::{Frequency, Length, ThermodynamicTemperature},
        frequency::hertz,
        length::{meter, nanometer},
    };

    use crate::formulae::constants::PEAK_FREQUENCY_CONSTANT;
    use crate::{
        formulae::constants::WIENS_DISPLACEMENT_CONSTANT, wavelength::wavelength::Wavelength,
    };

    /// Calculate the peak wavelength/frequency of an object based on it's temperature.
    /// Derived from Wien's Law.
    pub fn from_temperature(temperature: ThermodynamicTemperature) -> Wavelength {
        let wvlnth = Length::new::<meter>(WIENS_DISPLACEMENT_CONSTANT / temperature.value);
        let freq =
            Frequency::new::<kilohertz>(PEAK_FREQUENCY_CONSTANT * temperature.value / 1000.0);

        println!("FUNCTION GOT len {:?}", wvlnth);
        println!(
            "FUNCTION GOT fre {:?}",
            freq.into_format_args(gigahertz, Abbreviation)
        );

        Wavelength::new(wvlnth, freq)
    }
}

pub mod mass {
    use crate::formulae::constants::{BOLTZMANN_CONSTANT, GRAVITATIONAL_CONSTANT};
    use std::f64::consts;
    use uom::si::f64::{Mass, MassDensity, ThermodynamicTemperature, Volume};
    use uom::si::mass::kilogram;

    /// Mass that a spherical cloud of interstellar gas must have in order to contract under its own weight
    pub fn jeans_mass(
        temperature: ThermodynamicTemperature,
        mean_mass_per_particle: Mass,
        density: MassDensity,
    ) -> Mass {
        let c1 = density.value.powf(-1.0 / 2.0);
        let c2 = temperature.value.powf(3.0 / 2.0);
        let c3 = 5.0 * BOLTZMANN_CONSTANT;
        let c4 = GRAVITATIONAL_CONSTANT * mean_mass_per_particle.value;
        let res = c1 * c2 * ((c3 / c4).powf(3.0 / 2.0)) * f64::sqrt(3.0 / (4.0 * consts::PI));
        Mass::new::<kilogram>(res)
    }

    pub fn from_volume_and_density(volume: Volume, density: MassDensity) -> Mass {
        density * volume
    }
}

pub mod area {
    use std::f64::consts::PI;
    use uom::si::f64::{Area, Length};

    pub fn sphere_surface_from_radius(radius: Length) -> Area {
        4.0 * PI * radius.powi(crate::uom::typenum::P2::new())
    }
}

pub mod volume {
    use std::f64::consts::PI;
    use uom::si::f64::{Length, Volume};

    pub fn sphere_volume_from_length(radius: Length) -> Volume {
        4.0 / 3.0 * PI * radius.powi(crate::uom::typenum::P3::new())
    }
}

pub mod length {
    use std::f64::consts::PI;
    use uom::si::f64::{Length, Volume};
    use uom::si::length::meter;

    pub fn sphere_radius_from_volume(volume: Volume) -> Length {
        Length::new::<meter>(f64::cbrt(3.0 * volume.value / (4.0 * PI)))
    }
}

pub mod density {
    use crate::formulae::constants::GAS_CONSTANT;
    use uom::si::f64::{Mass, MassDensity, MolarMass, Pressure, ThermodynamicTemperature, Volume};
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::molar_mass::gram_per_mole;
    use uom::si::pressure::atmosphere;

    pub fn from_mass_and_volume(mass: Mass, volume: Volume) -> MassDensity {
        mass / volume
    }

    pub fn from_molar_mass_pressure_temperature(
        molar_mass: MolarMass,
        pressure: Pressure,
        temperature: ThermodynamicTemperature,
    ) -> MassDensity {
        MassDensity::new::<kilogram_per_cubic_meter>(
            ((molar_mass.value) * (pressure.value) / (GAS_CONSTANT * temperature.value)),
        )
    }
}

pub fn calculate_luminosity(
    emissivity: f64,
    surface_area: Area,
    temperature: ThermodynamicTemperature,
) -> Power {
    Power::new::<watt>(
        STEFAN_BOLTZMANN_CONSTANT
            * emissivity
            * surface_area.value
            * num::pow(temperature.value, 4),
    )
}

pub fn calculate_colour(temperature: ThermodynamicTemperature) -> RGB {
    colortemp::temp_to_rgb(temperature.value as i64)
}

pub fn calculate_bv_index(temperature: ThermodynamicTemperature) -> f64 {
    (5601.0 / temperature.value).powf(1.5) - 0.4
}

pub fn bv_to_rgb(bv: f64) -> RGB {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut bv = bv;
    let mut t;

    if bv < -0.4 {
        bv = -0.4;
    }

    if bv > 2.0 {
        bv = 2.0;
    }

    if bv >= -0.40 && bv < 0.00 {
        t = (bv + 0.40) / 0.40;
        r = 0.61 + (0.11 * t) + (0.1 * t * t);
    } else if bv >= 0.00 && bv < 0.40 {
        t = bv / 0.40;
        r = 0.83 + (0.17 * t);
    } else if bv >= 0.40 && bv < 2.10 {
        r = 1.00;
    }

    if bv >= -0.40 && bv < 0.00 {
        t = (bv + 0.40) / 0.40;
        g = 0.70 + (0.07 * t) + (0.1 * t * t);
    } else if bv >= 0.00 && bv < 0.40 {
        t = bv / 0.40;
        g = 0.87 + (0.11 * t);
    } else if bv >= 0.40 && bv < 1.60 {
        t = (bv - 0.40) / (1.60 - 0.40);
        g = 0.98 - (0.16 * t);
    } else if bv >= 1.60 && bv < 2.00 {
        t = (bv - 1.60) / (2.00 - 1.60);
        g = 0.82 - (0.5 * t * t);
    }

    if bv >= -0.40 && bv < 0.40 {
        b = 1.00;
    } else if bv >= 0.40 && bv < 1.50 {
        t = (bv - 0.40) / (1.50 - 0.40);
        b = 1.00 - (0.47 * t) + (0.1 * t * t);
    } else if bv >= 1.50 && bv < 1.94 {
        t = (bv - 1.50) / (1.94 - 1.50);
        b = 0.63 - (0.6 * t * t);
    }

    RGB {
        r: r * 255.0,
        g: g * 255.0,
        b: b * 255.0,
    }
}

pub fn calculate_temperature(
    luminosity: Power,
    emissivity: f64,
    area: Area,
) -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(f64::pow(
        (luminosity.value / (STEFAN_BOLTZMANN_CONSTANT * emissivity * area.value)),
        0.25,
    ))
}

pub fn calculate_absolute_magnitude(luminosity: Power) -> f64 {
    -2.5 * (f64::log10(luminosity.value / ZERO_POINT_LUMINOSITY))
}

// pub fn stellar_lifespan(mass: Mass, randomness: Option<&mut Randomness>) -> Time {
//     let calc = Time::new::<year>(
//         f64::pow(10.0, 10.0) * f64::pow((Mass::new::<solar_mass>(1.0) / mass).value, 2.5),
//     );
//
//     let randomised = match randomness {
//         None => calc,
//         Some(mut prng) => {
//             let rn = ((prng.gen_range(500, 1200) as f64) / 1000.0);
//             calc * rn
//         }
//     };
//
//     if randomised < Time::new::<million_year>(11.0) {
//         Time::new::<million_year>(11.0)
//     } else {
//         randomised
//     }
// }
