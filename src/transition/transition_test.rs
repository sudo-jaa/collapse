#[cfg(test)]
mod tests {
    use uom::si::f64::{Length, ThermodynamicTemperature};
    use uom::si::length::light_year;
    use uom::si::thermodynamic_temperature::kelvin;
    use crate::{Element, Gas, Molecule, SiliconIsotope, volume};
    use crate::transition::transition::Transition;

    #[test]
    fn transition_gas_to_self() {
        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(100.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(7.0);
        let cloud_origin = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, vec!((Molecule::molecular_hydrogen(), 100.0)));


        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(100.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(7.0);
        let cloud_target = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, vec!((Molecule::molecular_hydrogen(), 100.0)));

        let interpolated = cloud_origin.interpolate(&cloud_target, 1.0, None);

        assert_eq!(interpolated.temperature, cloud_target.temperature);
    }

    #[test]
    fn transition_gas_to_different() {
        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(100.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(7.0);
        let cloud_origin = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, vec!((Molecule::molecular_hydrogen(), 100.0)));


        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(30.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(700.0);
        let cloud_target = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, vec!((Molecule::molecular_hydrogen(), 100.0)));

        let interpolated = cloud_origin.interpolate(&cloud_target, 0.0, None);
        assert_eq!(interpolated.temperature, cloud_origin.temperature);

        let interpolated = cloud_origin.interpolate(&cloud_target, 50.0, None);
        assert_eq!(interpolated.temperature, ThermodynamicTemperature::new::<kelvin>(353.5));

        let interpolated = cloud_origin.interpolate(&cloud_target, 100.0, None);
        assert_eq!(interpolated.temperature, cloud_target.temperature);
    }

    #[test]
    fn transition_gas_to_different_with_ease() {
        let ease: fn(f64) -> f64 = |input| {
            input * input * input * input
        };

        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(100.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(7.0);
        let cloud_origin = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, vec!((Molecule::molecular_hydrogen(), 100.0)));


        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(30.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(700.0);
        let cloud_target = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, vec!((Molecule::molecular_hydrogen(), 100.0)));

        let interpolated = cloud_origin.interpolate(&cloud_target, 0.0, Some(ease));
        println!("interpolate {:?}", interpolated.temperature);
        // assert_eq!(interpolated.temperature, cloud_origin.temperature);

        let interpolated = cloud_origin.interpolate(&cloud_target, 25.0, Some(ease));
        println!("interpolate {:?}", interpolated.temperature);


        let interpolated = cloud_origin.interpolate(&cloud_target, 50.0, Some(ease));
        println!("interpolate {:?}", interpolated.temperature);
        // assert_eq!(interpolated.temperature, ThermodynamicTemperature::new::<kelvin>(353.5));

        let interpolated = cloud_origin.interpolate(&cloud_target, 75.0, Some(ease));
        println!("interpolate {:?}", interpolated.temperature);

        let interpolated = cloud_origin.interpolate(&cloud_target, 85.0, Some(ease));
        println!("interpolate {:?}", interpolated.temperature);

        let interpolated = cloud_origin.interpolate(&cloud_target, 95.0, Some(ease));
        println!("interpolate {:?}", interpolated.temperature);

        let interpolated = cloud_origin.interpolate(&cloud_target, 100.0, Some(ease));
        println!("interpolate {:?}", interpolated.temperature);
        // assert_eq!(interpolated.temperature, cloud_target.temperature);
    }



}
