#[cfg(test)]
mod tests {
    use std::time::Instant;
    use float_cmp::assert_approx_eq;
    use uom::fmt::DisplayStyle::Abbreviation;
    use uom::si::amount_of_substance::mole;
    use uom::si::f64::{AmountOfSubstance, Length, Mass, MassDensity, MolarMass, Pressure, ThermodynamicTemperature, Volume};
    use uom::si::length::{light_year, meter, parsec};
    use uom::si::molar_mass::gram_per_mole;
    use uom::si::power::watt;
    use uom::si::pressure::pascal;
    use uom::si::thermodynamic_temperature::kelvin;
    use uom::si::volume::cubic_meter;
    use crate::cloud::cloud::{CloudOptions, MolecularCloud};
    use crate::{Coordinates, solar_mass};
    use crate::chemistry::elements::carbon::CarbonIsotope;
    use crate::chemistry::elements::elements::Element;
    use crate::chemistry::elements::hydrogen::HydrogenIsotope;
    use crate::chemistry::elements::silicon::SiliconIsotope;
    use crate::chemistry::molecules::molecules::Molecule;
    use crate::formulae::formulae::{moles, volume};
    use crate::gas::gas::Gas;

    #[test]
    fn cloud_volume_test() {
        let coords = Coordinates::new(1, 1, 1);

        // 1% of the ism is usually dust.
        // lets say 5% for dust in clouds,
        // 90% molecular hydrogen,
        // lots of CO
        let l1 = Length::new::<light_year>(30.0);
        let cloud = MolecularCloud::new(
            coords,
            l1,
            Gas::generate_massdensity(&Molecule::new(vec!((Element::Hydrogen(HydrogenIsotope::Hydrogen), 1))), 300.0),
            vec!(
                (Molecule::new(vec!((Element::Silicon(SiliconIsotope::Silicon), 1))), 1.5),
                (Molecule::new(vec!((Element::Carbon(CarbonIsotope::Carbon), 1))), 1.5),
                (Molecule::molecular_hydrogen(), 80.0),
                (Molecule::carbon_monoxide(), 17.0),
            ),
            CloudOptions {
                use_randomness: false,
                core_formation_chance: 80.0,
                core_formation_dropoff: 0.80,
            });

        println!("cloud: {:#?}", cloud);
    }

    #[test]
    fn molecular_cloud_pressure() {
        let vol = Volume::new::<cubic_meter>(1.0);
        let temp = ThermodynamicTemperature::new::<kelvin>(7.0);
        let substance = moles::from_pressure_volume_temperature(Pressure::new::<pascal>(1.0), vol, temp);
        assert_eq!(substance, AmountOfSubstance::new::<mole>(0.017181765006103723));

        println!("{:?}", moles::from_pressure_volume_temperature(MolecularCloud::pressure(), volume::sphere_volume_from_length(Length::new::<light_year>(25.0)), temp));

        // println!("{:?}", MolecularCloud::moles(Pressure::new::<pascal>(1.0), vol, temp));
        // let moles = MolarMass::new::<gram_per_mole>(1.0);
        // println!("{:?}", MolecularCloud::pressure(vol, moles, temp));
    }
}
