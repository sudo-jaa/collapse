#[cfg(test)]
mod tests {
    use uom::fmt::DisplayStyle::Abbreviation;
    use uom::si::amount_of_substance::mole;
    use uom::si::f64::{AmountOfSubstance, Mass, Pressure, ThermodynamicTemperature, Volume, MassDensity, Length, ThermalConductivity};
    use uom::si::length::{light_year, parsec};
    use uom::si::mass::{gram, kilogram};
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::pressure::{atmosphere, pascal};
    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
    use uom::si::volume::cubic_meter;
    use crate::chemistry::molecules::molecules::Molecule;
    use crate::formulae::formulae::volume;
    use crate::gas::gas::Gas;
    use crate::{Composition, solar_mass};
    use crate::chemistry::elements::elements::Element;
    use crate::chemistry::elements::silicon::SiliconIsotope;

    #[test]
    fn test_gas_from_ideal_PVT() {
        let molecule = Molecule::molecular_hydrogen();
        let pressure = Pressure::new::<pascal>(1.0);
        let volume: Volume = Volume::new::<cubic_meter>(1.0);
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(1.0);
        let gas = Gas::from_pressure_volume_temperature(pressure, volume, temperature, molecule);

        assert_eq!(gas.moles, AmountOfSubstance::new::<mole>(0.12027235504272604));
    }

    #[test]
    fn test_gas_from_ideal_PVM() {
        let molecule = Molecule::molecular_hydrogen();
        let pressure = Pressure::new::<pascal>(1.0);
        let volume: Volume = Volume::new::<cubic_meter>(1.0);
        let moles: AmountOfSubstance = AmountOfSubstance::new::<mole>(1.0);
        let gas = Gas::from_pressure_volume_moles(pressure, volume, moles, molecule);

        assert_eq!(gas.temperature, ThermodynamicTemperature::new::<kelvin>(0.12027235504272604));
    }

    #[test]
    fn test_gas_from_ideal_VMT() {
        let molecule = Molecule::molecular_hydrogen();
        let volume: Volume = Volume::new::<cubic_meter>(1.0);
        let moles: AmountOfSubstance = AmountOfSubstance::new::<mole>(1.0);
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(1.0);

        let gas = Gas::from_volume_moles_temperature(volume, moles, temperature, molecule);
        assert_eq!(gas.pressure, Pressure::new::<pascal>(8.31446261815324));
    }

    #[test]
    fn test_gas_from_ideal_PMT() {
        let molecule = Molecule::molecular_hydrogen();
        let pressure = Pressure::new::<pascal>(1.0);
        let moles: AmountOfSubstance = AmountOfSubstance::new::<mole>(1.0);
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(1.0);

        let gas = Gas::from_pressure_moles_temperature(pressure, moles, temperature, molecule);
        assert_eq!(gas.volume, Volume::new::<cubic_meter>(8.31446261815324));
    }

    #[test]
    fn test_mass_of_gas() {
        let molecule = Molecule::carbon_monoxide();
        let pressure = Pressure::new::<pascal>(1.0);
        let moles: AmountOfSubstance = AmountOfSubstance::new::<mole>(1.0);
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(1.0);

        let gas = Gas::from_pressure_moles_temperature(pressure, moles, temperature, molecule);

        assert_eq!(gas.mass, Mass::new::<kilogram>(0.027994900000000003));
    }

    #[test]
    fn test_density_of_gas() {
        let molecule = Molecule::carbon_monoxide();
        let pressure = Pressure::new::<atmosphere>(0.5);
        let moles: AmountOfSubstance = AmountOfSubstance::new::<mole>(32.0);
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<degree_celsius>(27.0);

        let gas = Gas::from_pressure_moles_temperature(pressure, moles, temperature, molecule);
        assert_eq!(gas.density, MassDensity::new::<kilogram_per_cubic_meter>(0.5683200847079912))
    }

    #[test]
    fn test_molecular_clouds() {
        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(100.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(7.0);
        let cloud = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, Composition(vec!(
            (Molecule::molecular_hydrogen(), 84.0),
            (Molecule::carbon_monoxide(), 10.0),
            (Molecule::atomic_helium(), 5.0),
            (Molecule::new(
                vec!(
                    (Element::Silicon(SiliconIsotope::Silicon), 1)
                )
            ), 1.0)
        )));

        println!("mass of cloud: {}", cloud.mass.into_format_args(solar_mass, Abbreviation));
    }

    #[test]
    fn test_composite_density() {
        let target_density = Gas::generate_massdensity(&Molecule::atomic_hydrogen(), 300.0);
        let density = Gas::generate_composite_massdensity(&Composition(vec!((Molecule::atomic_hydrogen(), 100.0))), 300.0);
        assert_eq!(density, target_density);

        let target_density = Gas::generate_massdensity(&Molecule::atomic_hydrogen(), 300.0);
        let density = Gas::generate_composite_massdensity(&Composition(vec!((Molecule::atomic_hydrogen(), 50.0), (Molecule::atomic_hydrogen(), 50.0))), 300.0);
        assert_eq!(density, target_density);

        let target_density = Gas::generate_massdensity(&Molecule::atomic_hydrogen(), 300.0);
        let density = Gas::generate_composite_massdensity(&Composition(vec!((Molecule::atomic_hydrogen(), 50.0), (Molecule::atomic_hydrogen(), 50.0))), 300.0);
        assert_eq!(density, target_density);

        let target_density = Gas::generate_massdensity(&Molecule::molecular_hydrogen(), 300.0);
        let density = Gas::generate_composite_massdensity(&Composition(vec!((Molecule::atomic_hydrogen(), 100.0), (Molecule::atomic_hydrogen(), 100.0))), 300.0);
        assert_eq!(density, target_density);
    }

    #[test]
    fn test_collapsability() {
        let target_density = Gas::generate_massdensity(&Molecule::molecular_hydrogen(), 300.0);

        let vacuum_pressure = Pressure::new::<pascal>(1.322e-11);
        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(150.0));
        let temperature = ThermodynamicTemperature::new::<kelvin>(7.0);

        let molecule = Molecule::molecular_hydrogen();
        let gas = Gas::from_vacuum_properties(volume, 300.0, temperature, molecule);

        println!("mass: {:?}", gas.mass.into_format_args(solar_mass, Abbreviation));
        println!("density: {:?}", gas.density.value);
        println!("substance: {:?}", gas.moles);
        println!("stable? {:?}", gas.stable());
    }

    #[test]
    fn density_test() {
        let density = Gas::generate_massdensity(&Molecule::molecular_hydrogen(), 1.0);
        assert_eq!(density, MassDensity::new::<kilogram_per_cubic_meter>(3.346964268002621e-27));
    }
}
