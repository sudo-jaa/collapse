#[cfg(test)]
mod tests {
    extern crate uom;

    use float_cmp::{approx_eq, assert_approx_eq};
    use uom::si::frequency::terahertz;

    use crate::formulae::constants::VACUUM_PERMEABILITY;
    use crate::formulae::formulae::{
        area, calculate_absolute_magnitude, calculate_colour, calculate_luminosity,
        calculate_temperature, density, mass, volume, wavelength,
    };
    use crate::units::units::length::{earth_radius, solar_radius};
    use crate::units::units::mass::{earth_mass, solar_mass};
    use crate::units::units::power::solar_luminosity;
    use crate::wavelength::wavelength::Wavelength;
    use colortemp::RGB;
    use uom::fmt::DisplayStyle::Abbreviation;
    use uom::num_traits::pow::pow;
    use uom::si::area::square_kilometer;
    use uom::si::area::square_meter;
    use uom::si::f64::*;
    use uom::si::inductance::henry;
    use uom::si::information::mebibit;
    use uom::si::length::{kilometer, meter, nanometer};
    use uom::si::mass::{gram, kilogram, nanogram};
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::power::gigawatt;
    use uom::si::thermodynamic_temperature::kelvin;
    use uom::si::volume::cubic_meter;

    #[test]
    fn formula_surface_area() {
        let surface = area::sphere_surface_from_radius(Length::new::<meter>(1.0));
        assert_eq!(surface, Area::new::<square_meter>(12.566370614359172));

        let earth = area::sphere_surface_from_radius(Length::new::<earth_radius>(1.0));
        assert_eq!(earth, Area::new::<square_meter>(510064471909788.25));
    }

    #[test]
    fn test_wiens_law() {
        let wv: Wavelength =
            wavelength::from_temperature(ThermodynamicTemperature::new::<kelvin>(1.0));
        assert_approx_eq!(
            f64,
            wv.peak_wavelength.value,
            Length::new::<nanometer>(2897771.9549999996).value,
            ulps = 2
        );
        assert_approx_eq!(
            f64,
            wv.peak_frequency.value,
            Frequency::new::<terahertz>(0.058789232).value,
            ulps = 2
        );

        let wv: Wavelength =
            wavelength::from_temperature(ThermodynamicTemperature::new::<kelvin>(5778.0));

        assert_approx_eq!(
            f64,
            wv.peak_wavelength.value,
            Length::new::<nanometer>(501.5181645898235).value,
            ulps = 2
        );
        assert_approx_eq!(
            f64,
            wv.peak_frequency.value,
            Frequency::new::<terahertz>(339.684182496000).value,
            ulps = 2
        );
    }

    #[test]
    fn formula_luminosity() {
        let r = Length::new::<kilometer>(695700.0);
        let sa = area::sphere_surface_from_radius(r);
        let temp = ThermodynamicTemperature::new::<kelvin>(5778.0);

        let lum = calculate_luminosity(1.0, sa, temp);
        let cmp_lum = Power::new::<solar_luminosity>(1.0);
        assert!(approx_eq!(f64, lum.value, cmp_lum.value, ulps = 11));
    }

    #[test]
    fn formula_temperature_to_col() {
        let temp = ThermodynamicTemperature::new::<kelvin>(5778.0);
        let col = calculate_colour(temp);
        assert_eq!(
            col,
            RGB {
                r: 255.0,
                g: 241.0,
                b: 228.0
            },
            "yellow star"
        );

        let temp = ThermodynamicTemperature::new::<kelvin>(41000.0);
        let col = calculate_colour(temp);
        assert_eq!(
            col,
            RGB {
                r: 0.0,
                g: 185.0,
                b: 255.0
            },
            "blue star"
        );

        let temp = ThermodynamicTemperature::new::<kelvin>(3850.0);
        let col = calculate_colour(temp);
        assert_eq!(
            col,
            RGB {
                r: 255.0,
                g: 201.0,
                b: 157.0
            },
            "red star"
        );
    }

    #[test]
    fn formula_density() {
        let mass = Mass::new::<kilogram>(1.0);
        let radius = Length::new::<meter>(1.0);
        let volume = volume::sphere_volume_from_length(radius);
        let density = density::from_mass_and_volume(mass, volume);
        assert_eq!(volume, Volume::new::<cubic_meter>(4.1887902047863905));
        assert_eq!(
            density,
            MassDensity::new::<kilogram_per_cubic_meter>(0.23873241463784303)
        );

        let mass = Mass::new::<earth_mass>(1.0);
        let radius = Length::new::<earth_radius>(1.0);
        let volume = volume::sphere_volume_from_length(radius);
        let density = density::from_mass_and_volume(mass, volume);
        assert_eq!(volume, Volume::new::<cubic_meter>(1083206916845753600000.0));
        assert_eq!(
            density,
            MassDensity::new::<kilogram_per_cubic_meter>(5516.95147719498)
        );
    }

    #[test]
    fn formula_temperature() {
        let luminosity = Power::new::<solar_luminosity>(1.0);
        let radius = Length::new::<solar_radius>(1.0);
        let area = area::sphere_surface_from_radius(radius);
        assert_eq!(
            calculate_temperature(luminosity, 1.0, area),
            ThermodynamicTemperature::new::<kelvin>(5778.0)
        );

        let luminosity = Power::new::<solar_luminosity>(2.0);
        let radius = Length::new::<solar_radius>(1.0);
        let area = area::sphere_surface_from_radius(radius);
        assert_eq!(
            calculate_temperature(luminosity, 1.0, area),
            ThermodynamicTemperature::new::<kelvin>(6871.238710485723)
        );

        let luminosity = Power::new::<solar_luminosity>(15.0);
        let radius = Length::new::<solar_radius>(5.0);
        let area = area::sphere_surface_from_radius(radius);
        assert_eq!(
            calculate_temperature(luminosity, 1.0, area),
            ThermodynamicTemperature::new::<kelvin>(5085.285615192227)
        );

        let luminosity = Power::new::<solar_luminosity>(30.0);
        let radius = Length::new::<solar_radius>(0.2);
        let area = area::sphere_surface_from_radius(radius);
        assert_eq!(
            calculate_temperature(luminosity, 1.0, area),
            ThermodynamicTemperature::new::<kelvin>(30237.28917703793)
        );
    }

    #[test]
    fn formula_abs_magnitude() {
        let lum = Power::new::<solar_luminosity>(1.0);
        let mag = calculate_absolute_magnitude(lum);
        assert_eq!(mag, 4.735487783751088);

        let lum = Power::new::<solar_luminosity>(12.0);
        let mag = calculate_absolute_magnitude(lum);
        assert_eq!(mag, 2.0375346686320266);

        let lum = Power::new::<solar_luminosity>(25.0);
        let mag = calculate_absolute_magnitude(lum);
        assert_eq!(mag, 1.2406377620709945);

        let lum = Power::new::<solar_luminosity>(75.0);
        let mag = calculate_absolute_magnitude(lum);
        assert_eq!(mag, 0.04783462527183831);

        let lum = Power::new::<solar_luminosity>(250.0);
        let mag = calculate_absolute_magnitude(lum);
        assert_eq!(mag, -1.2593622379290057);
    }

    #[test]
    fn formula_mass_from_density_and_volume() {
        assert_eq!(
            mass::from_volume_and_density(
                Volume::new::<cubic_meter>(20.0),
                MassDensity::new::<kilogram_per_cubic_meter>(100.0)
            ),
            Mass::new::<gram>(2000000.0)
        );
    }

    #[test]
    fn jeans_mass_test() {
        use approx_eq;

        let temperature = ThermodynamicTemperature::new::<kelvin>(50.0);
        let particle_mean = Mass::new::<gram>(1.0);
        let density = MassDensity::new::<kilogram_per_cubic_meter>(1.0);
        let jeans_mass = mass::jeans_mass(temperature, particle_mean, density);

        let val = jeans_mass.value;
        let cmp = Mass::new::<nanogram>(5.746).value;
        println!("{} {}", val, cmp);
        assert!(approx_eq!(f64, val, cmp, epsilon = 0.00000000000003));
    }

    #[test]
    fn vacuum_permeability_test() {
        let vp = Inductance::new::<henry>(VACUUM_PERMEABILITY);

        println!("{:?}", vp);
    }
}
