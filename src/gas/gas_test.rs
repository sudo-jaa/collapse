#[cfg(test)]
mod tests {
    use crate::chemistry::elements::elements::Element;
    use crate::chemistry::elements::silicon::SiliconIsotope;
    use crate::chemistry::molecules::molecules::Molecule;
    use crate::formulae::formulae::volume;
    use crate::gas::gas::UniformGas;
    use crate::{solar_mass, Composition};
    use uom::fmt::DisplayStyle::Abbreviation;
    use uom::si::amount_of_substance::mole;
    use uom::si::f64::{
        AmountOfSubstance, Length, Mass, MassDensity, Pressure, ThermalConductivity,
        ThermodynamicTemperature, Volume,
    };
    use uom::si::length::{light_year, meter, parsec};
    use uom::si::mass::{gram, kilogram};
    use uom::si::mass_density::{femtogram_per_cubic_meter, kilogram_per_cubic_meter};
    use uom::si::pressure::{atmosphere, pascal};
    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
    use uom::si::volume::cubic_meter;

    // #[test]
    // fn observation_test_molecular_clouds() {
    //     let volume = volume::sphere_volume_from_length(Length::new::<light_year>(100.0));
    //     let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(7.0);
    //     let cloud = UniformGas::composite_from_vacuum_properties(
    //         volume,
    //         100000000.0,
    //         temperature,
    //         Composition(vec![
    //             (Molecule::molecular_hydrogen(), 84.0),
    //             (Molecule::carbon_monoxide(), 10.0),
    //             (Molecule::atomic_helium(), 5.0),
    //             (
    //                 Molecule::new(vec![(Element::Silicon(SiliconIsotope::Silicon), 1)]),
    //                 1.0,
    //             ),
    //         ]),
    //     );

    //     println!(
    //         "mass of cloud: {}",
    //         cloud.mass.into_format_args(solar_mass, Abbreviation)
    //     );
    // }

    #[test]
    fn observation_test_giant_molecular_clouds_properties() {
        // These giant molecular clouds have typical densities of 100 particles per cm3, diameters of 100 light-years (9.5×1014 km), masses of up to 6 million solar masses (M☉)
        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(100.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(10.0);
        let cloud = UniformGas::composite_from_vacuum_properties(
            volume,
            300000000.0,
            temperature,
            Composition(vec![
                (Molecule::molecular_hydrogen(), 84.0),
                (Molecule::carbon_monoxide(), 10.0),
                (Molecule::atomic_helium(), 5.0),
                (
                    Molecule::new(vec![(Element::Silicon(SiliconIsotope::Silicon), 1)]),
                    1.0,
                ),
            ]),
        );

        assert!(
            cloud.mass > Mass::new::<solar_mass>(1000000.0)
                && cloud.mass < Mass::new::<solar_mass>(8000000.0)
        );
    }

    #[test]
    fn function_resize_gas_energy_release_test() {
        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(100.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(10.0);
        let cloud = UniformGas::composite_from_vacuum_properties(
            volume,
            300000000.0,
            temperature,
            Composition(vec![
                (Molecule::molecular_hydrogen(), 84.0),
                (Molecule::carbon_monoxide(), 10.0),
                (Molecule::atomic_helium(), 5.0),
                (
                    Molecule::new(vec![(Element::Silicon(SiliconIsotope::Silicon), 1)]),
                    1.0,
                ),
            ]),
        );

        let advanced =
            cloud.clone().resize(volume::sphere_volume_from_length(
                Length::new::<light_year>(50.0),
            ));

        // assert_eq!(cloud.moles, advanced.moles);
        // assert_eq!(cloud.volume, advanced.volume);
        // assert_eq!(cloud.density, advaned.density);
    }

    #[test]
    fn function_resize_gas_density_test() {
        let volume = volume::sphere_volume_from_length(Length::new::<meter>(1.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(10.0);
        let cloud = UniformGas::composite_from_vacuum_properties(
            volume,
            300000000.0,
            temperature,
            Composition(vec![
                (Molecule::molecular_hydrogen(), 84.0),
                (Molecule::carbon_monoxide(), 10.0),
                (Molecule::atomic_helium(), 5.0),
                (
                    Molecule::new(vec![(Element::Silicon(SiliconIsotope::Silicon), 1)]),
                    1.0,
                ),
            ]),
        );

        let advanced = cloud
            .clone()
            .resize(volume::sphere_volume_from_length(Length::new::<meter>(1.0)));

        assert_eq!(cloud.moles, advanced.0.moles);
        assert_eq!(cloud.volume, advanced.0.volume);
        assert_eq!(cloud.density, advanced.0.density);

        let volume = volume::sphere_volume_from_length(Length::new::<meter>(1.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(10.0);
        let cloud = UniformGas::composite_from_vacuum_properties(
            volume,
            300000000.0,
            temperature,
            Composition(vec![
                (Molecule::molecular_hydrogen(), 84.0),
                (Molecule::carbon_monoxide(), 10.0),
                (Molecule::atomic_helium(), 5.0),
                (
                    Molecule::new(vec![(Element::Silicon(SiliconIsotope::Silicon), 1)]),
                    1.0,
                ),
            ]),
        );

        let advanced = cloud
            .clone()
            .resize(volume::sphere_volume_from_length(Length::new::<meter>(0.5)));

        assert_eq!(cloud.moles, advanced.0.moles);
        assert_eq!(
            advanced.0.density,
            MassDensity::new::<kilogram_per_cubic_meter>(cloud.density.value * 1.875)
        );
    }

    #[test]
    fn function_test_gas_from_ideal_pvt() {
        let molecule = Molecule::molecular_hydrogen();
        let pressure = Pressure::new::<pascal>(1.0);
        let volume: Volume = Volume::new::<cubic_meter>(1.0);
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(1.0);
        let gas =
            UniformGas::from_pressure_volume_temperature(pressure, volume, temperature, molecule);

        assert_eq!(
            gas.moles,
            AmountOfSubstance::new::<mole>(0.12027235504272604)
        );
    }

    #[test]
    fn function_test_gas_from_ideal_pvm() {
        let molecule = Molecule::molecular_hydrogen();
        let pressure = Pressure::new::<pascal>(1.0);
        let volume: Volume = Volume::new::<cubic_meter>(1.0);
        let moles: AmountOfSubstance = AmountOfSubstance::new::<mole>(1.0);
        let gas = UniformGas::from_pressure_volume_moles(pressure, volume, moles, molecule);

        assert_eq!(
            gas.temperature,
            ThermodynamicTemperature::new::<kelvin>(0.12027235504272604)
        );
    }

    #[test]
    fn function_test_gas_from_ideal_vmt() {
        let molecule = Molecule::molecular_hydrogen();
        let volume: Volume = Volume::new::<cubic_meter>(1.0);
        let moles: AmountOfSubstance = AmountOfSubstance::new::<mole>(1.0);
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(1.0);

        let gas = UniformGas::from_volume_moles_temperature(volume, moles, temperature, molecule);
        assert_eq!(gas.pressure, Pressure::new::<pascal>(8.31446261815324));
    }

    #[test]
    fn function_test_gas_from_ideal_pmt() {
        let molecule = Molecule::molecular_hydrogen();
        let pressure = Pressure::new::<pascal>(1.0);
        let moles: AmountOfSubstance = AmountOfSubstance::new::<mole>(1.0);
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(1.0);

        let gas =
            UniformGas::from_pressure_moles_temperature(pressure, moles, temperature, molecule);
        assert_eq!(gas.volume, Volume::new::<cubic_meter>(8.31446261815324));
    }

    #[test]
    fn function_test_mass_of_gas() {
        let molecule = Molecule::carbon_monoxide();
        let pressure = Pressure::new::<pascal>(1.0);
        let moles: AmountOfSubstance = AmountOfSubstance::new::<mole>(1.0);
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(1.0);

        let gas =
            UniformGas::from_pressure_moles_temperature(pressure, moles, temperature, molecule);

        assert_eq!(gas.mass, Mass::new::<kilogram>(0.027994900000000003));
    }

    #[test]
    fn function_test_density_of_gas() {
        let molecule = Molecule::carbon_monoxide();
        let pressure = Pressure::new::<atmosphere>(0.5);
        let moles: AmountOfSubstance = AmountOfSubstance::new::<mole>(32.0);
        let temperature: ThermodynamicTemperature =
            ThermodynamicTemperature::new::<degree_celsius>(27.0);

        let gas =
            UniformGas::from_pressure_moles_temperature(pressure, moles, temperature, molecule);
        assert_eq!(
            gas.density,
            MassDensity::new::<kilogram_per_cubic_meter>(0.5683200847079912)
        )
    }

    #[test]
    fn function_test_composite_density() {
        let target_density = UniformGas::generate_massdensity(&Molecule::atomic_hydrogen(), 300.0);
        let density = UniformGas::generate_composite_massdensity(
            &Composition(vec![(Molecule::atomic_hydrogen(), 100.0)]),
            300.0,
        );
        assert_eq!(density, target_density);

        let target_density = UniformGas::generate_massdensity(&Molecule::atomic_hydrogen(), 300.0);
        let density = UniformGas::generate_composite_massdensity(
            &Composition(vec![
                (Molecule::atomic_hydrogen(), 50.0),
                (Molecule::atomic_hydrogen(), 50.0),
            ]),
            300.0,
        );
        assert_eq!(density, target_density);

        let target_density = UniformGas::generate_massdensity(&Molecule::atomic_hydrogen(), 300.0);
        let density = UniformGas::generate_composite_massdensity(
            &Composition(vec![
                (Molecule::atomic_hydrogen(), 50.0),
                (Molecule::atomic_hydrogen(), 50.0),
            ]),
            300.0,
        );
        assert_eq!(density, target_density);

        let target_density =
            UniformGas::generate_massdensity(&Molecule::molecular_hydrogen(), 300.0);
        let density = UniformGas::generate_composite_massdensity(
            &Composition(vec![
                (Molecule::atomic_hydrogen(), 100.0),
                (Molecule::atomic_hydrogen(), 100.0),
            ]),
            300.0,
        );
        assert_eq!(density, target_density);
    }

    #[test]
    fn function_test_collapsability() {
        let target_density =
            UniformGas::generate_massdensity(&Molecule::molecular_hydrogen(), 300.0);

        let vacuum_pressure = Pressure::new::<pascal>(1.322e-11);
        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(150.0));
        let temperature = ThermodynamicTemperature::new::<kelvin>(7.0);

        let molecule = Molecule::molecular_hydrogen();
        let gas = UniformGas::from_vacuum_properties(volume, 300.0, temperature, molecule);

        println!(
            "mass: {:?}",
            gas.mass.into_format_args(solar_mass, Abbreviation)
        );
        println!("density: {:?}", gas.density.value);
        println!("substance: {:?}", gas.moles);
        println!("stable? {:?}", gas.stable());
    }

    #[test]
    fn function_density_test() {
        let density = UniformGas::generate_massdensity(&Molecule::molecular_hydrogen(), 1.0);
        assert_eq!(
            density,
            MassDensity::new::<kilogram_per_cubic_meter>(3.346964268002621e-27)
        );
    }
}
