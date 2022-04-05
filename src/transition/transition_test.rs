#[cfg(test)]
mod tests {
    use uom::si::f64::{Length, ThermodynamicTemperature};
    use uom::si::length::light_year;
    use uom::si::thermodynamic_temperature::kelvin;
    use crate::{Composition, Element, Gas, Molecule, SiliconIsotope, volume};
    use crate::transition::transition::{Interpolatable };

    #[test]
    fn transition_composition_with_parity_test() {
        let c1 = Composition(vec!((Molecule::molecular_hydrogen(), 80.0), (Molecule::atomic_helium(), 20.0)));
        let c2 = Composition(vec!((Molecule::molecular_hydrogen(), 20.0), (Molecule::atomic_helium(), 80.0)));

        let start = c1.interpolate(&c2, 0.0, None);
        assert!(start.0.contains(&(Molecule::molecular_hydrogen(), 80.0)));
        assert!(start.0.contains(&(Molecule::atomic_helium(), 20.0)));


        let mid = c1.interpolate(&c2, 50.0, None);
        assert!(mid.0.contains(&(Molecule::molecular_hydrogen(), 50.0)));
        assert!(mid.0.contains(&(Molecule::atomic_helium(), 50.0)));


        let end = c1.interpolate(&c2, 100.0, None);
        assert!(end.0.contains(&(Molecule::molecular_hydrogen(), 20.0)));
        assert!(end.0.contains(&(Molecule::atomic_helium(), 80.0)));
    }

    #[test]
    fn transition_composition_with_right_imbalance_test() {
        let c1 = Composition(vec!((Molecule::molecular_hydrogen(), 100.0)));
        let c2 = Composition(vec!((Molecule::molecular_hydrogen(), 50.0), (Molecule::atomic_helium(), 50.0)));

        let start = c1.interpolate(&c2, 0.0, None);
        assert!(start.0.contains(&(Molecule::molecular_hydrogen(), 100.0)));
        assert!(start.0.contains(&(Molecule::atomic_helium(), 0.0)));

        let end = c1.interpolate(&c2, 100.0, None);
        assert!(end.0.contains(&(Molecule::molecular_hydrogen(), 50.0)));
        assert!(end.0.contains(&(Molecule::atomic_helium(), 50.0)));

        let mid = c1.interpolate(&c2, 50.0, None);
        assert!(mid.0.contains(&(Molecule::molecular_hydrogen(), 75.0)));
        assert!(mid.0.contains(&(Molecule::atomic_helium(), 25.0)));
    }

    #[test]
    fn transition_composition_with_left_imbalance_test() {
        let c1 = Composition(vec!((Molecule::molecular_hydrogen(), 50.0), (Molecule::atomic_helium(), 50.0)));
        let c2 = Composition(vec!((Molecule::molecular_hydrogen(), 100.0)));

        let start = c1.interpolate(&c2, 0.0, None);
        assert!(start.0.contains(&(Molecule::molecular_hydrogen(), 50.0)));
        assert!(start.0.contains(&(Molecule::atomic_helium(), 50.0)));

        let end = c1.interpolate(&c2, 100.0, None);
        assert!(end.0.contains(&(Molecule::molecular_hydrogen(), 100.0)));
        assert!(end.0.contains(&(Molecule::atomic_helium(), 0.0)));

        let mid = c1.interpolate(&c2, 50.0, None);
        assert!(mid.0.contains(&(Molecule::molecular_hydrogen(), 75.0)));
        assert!(mid.0.contains(&(Molecule::atomic_helium(), 25.0)));
    }

    #[test]
    fn transition_gas_to_self() {
        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(100.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(7.0);
        let cloud_origin = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, Composition(vec!((Molecule::molecular_hydrogen(), 100.0))));


        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(100.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(7.0);
        let cloud_target = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, Composition(vec!((Molecule::molecular_hydrogen(), 100.0))));

        let interpolated = cloud_origin.interpolate(&cloud_target, 1.0, None);

        assert_eq!(interpolated.temperature, cloud_target.temperature);
    }

    #[test]
    fn transition_gas_to_different() {
        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(100.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(7.0);
        let cloud_origin = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, Composition(vec!((Molecule::molecular_hydrogen(), 100.0))));


        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(30.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(700.0);
        let cloud_target = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, Composition(vec!((Molecule::molecular_hydrogen(), 100.0))));

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
        let cloud_origin = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, Composition(vec!((Molecule::molecular_hydrogen(), 100.0))));


        let volume = volume::sphere_volume_from_length(Length::new::<light_year>(30.0));
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(700.0);
        let cloud_target = Gas::composite_from_vacuum_properties(volume, 800.0, temperature, Composition(vec!((Molecule::molecular_hydrogen(), 80.0), (Molecule::atomic_helium(), 20.0))));

        let interpolated = cloud_origin.interpolate(&cloud_target, 0.0, Some(ease));
        println!();
        println!("interpolate {:?}", interpolated);
        // assert_eq!(interpolated.temperature, cloud_origin.temperature);

        let interpolated = cloud_origin.interpolate(&cloud_target, 25.0, Some(ease));
        println!();
        println!("interpolate {:?}", interpolated);
        // println!("interpolate {:?}", interpolated.temperature);


        let interpolated = cloud_origin.interpolate(&cloud_target, 50.0, Some(ease));
        println!();
        println!("interpolate {:?}", interpolated);
        // println!("interpolate {:?}", interpolated.temperature);
        // assert_eq!(interpolated.temperature, ThermodynamicTemperature::new::<kelvin>(353.5));

        let interpolated = cloud_origin.interpolate(&cloud_target, 75.0, Some(ease));
        println!();
        println!("interpolate {:?}", interpolated);
        // println!("interpolate {:?}", interpolated.temperature);

        let interpolated = cloud_origin.interpolate(&cloud_target, 85.0, Some(ease));
        println!();
        println!("interpolate {:?}", interpolated);
        // println!("interpolate {:?}", interpolated.temperature);

        let interpolated = cloud_origin.interpolate(&cloud_target, 95.0, Some(ease));
        println!();
        println!("interpolate {:?}", interpolated);
        // println!("interpolate {:?}", interpolated.temperature);

        let interpolated = cloud_origin.interpolate(&cloud_target, 100.0, Some(ease));
        println!();
        println!("interpolate {:?}", interpolated);
        // println!("interpolate {:?}", interpolated.temperature);
        // assert_eq!(interpolated.temperature, cloud_target.temperature);
    }

    #[test]
    fn interpolate_composition() {

    }



}
