#[cfg(test)]
mod tests {
    extern crate uom;
    use crate::units::units::length::earth_radius;
    use crate::units::units::mass::{earth_mass, solar_mass};
    use crate::units::units::power::solar_luminosity;
    use crate::units::units::time::{billion_year, million_year};
    use uom::si::area::square_kilometer;
    use uom::si::area::square_meter;
    use uom::si::f64::*;
    use uom::si::length::astronomical_unit;
    use uom::si::length::kilometer;
    use uom::si::length::meter;
    use uom::si::mass::kilogram;
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::power::gigawatt;
    use uom::si::pressure::{atmosphere, pascal};
    use uom::si::thermodynamic_temperature::kelvin;
    use uom::si::time::{second, year};
    use uom::si::volume::cubic_meter;
    use crate::formulae::constants::{INTERSTELLAR_MEDIUM_PARTICLE_DENSITY_HIGH, INTERSTELLAR_MEDIUM_PARTICLE_DENSITY_LOW, INTERSTELLAR_MEDIUM_PARTICLE_DENSITY_MID};

    #[test]
    fn solar_luminosity_initialises_correctly() {
        let s = Power::new::<solar_luminosity>(1.0);
        let gw = Power::new::<gigawatt>(384392751142533100.0);
        assert_eq!(s, gw);
    }

    #[test]
    fn earth_mass_initialises_correctly() {
        let e = Mass::new::<earth_mass>(1.0);
        let m = Mass::new::<kilogram>(5976000000000000000000000.0);
        assert_eq!(e, m);
    }

    #[test]
    fn earth_radius_initialises_correctly() {
        let e = Length::new::<earth_radius>(1.0);
        let m = Length::new::<meter>(6371000.0);
        assert_eq!(e, m);
    }

    #[test]
    fn years_initialises_correctly() {
        let my = Time::new::<million_year>(1.0);
        assert_eq!(my, Time::new::<year>(1000000.0), "million years is correct");

        let my = Time::new::<billion_year>(1.0);
        assert_eq!(
            my,
            Time::new::<year>(1000000000.0),
            "billion years is correct"
        );
        assert_eq!(
            my,
            Time::new::<million_year>(1000.0),
            "billion years compared to millions is correct"
        );
    }

    #[test]
    fn hydrogen_cosmos_density() {
        // let h_low = MassDensity::new::<hydrogen_atom_per_cubic_centimeter>(INTERSTELLAR_MEDIUM_PARTICLE_DENSITY_LOW);
        // let h_mid = MassDensity::new::<hydrogen_atom_per_cubic_centimeter>(INTERSTELLAR_MEDIUM_PARTICLE_DENSITY_MID);
        // let h_high = MassDensity::new::<hydrogen_atom_per_cubic_centimeter>(INTERSTELLAR_MEDIUM_PARTICLE_DENSITY_HIGH);
        // println!("{:?}", h_low);
        // println!("{:?}", h_mid);
        // println!("{:?}", h_high);
    }

    #[test]
    fn volume_and_density_to_mass() {
        let v = Volume::new::<cubic_meter>(6.0);
        let d = MassDensity::new::<kilogram_per_cubic_meter>(10.0);
        println!("{:?}", Mass::new::<kilogram>(v.value * d.value));
    }

    #[test]
    fn test_pascals_to_atmosphere() {
        assert_eq!(Pressure::new::<atmosphere>(1.0), Pressure::new::<pascal>(101325.0))
    }

}
