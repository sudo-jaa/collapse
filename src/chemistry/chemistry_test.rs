#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use uom::si::f64::Mass;
    use uom::si::power::watt;
    use crate::units::units::mass::dalton;
    use crate::chemistry::elements::elements::Element;
    use crate::chemistry::elements::helium::HeliumIsotope;
    use crate::chemistry::elements::hydrogen::HydrogenIsotope;
    use crate::chemistry::elements::oxygen::OxygenIsotope;

    #[test]
    fn test_atomic_mass_of_elements() {
        let hydrogen = Element::Hydrogen(HydrogenIsotope::Hydrogen);

        // Hydrogen mass in kg
        assert_approx_eq!(f64, hydrogen.data().atomic_mass().value, 1.6734821340013104e-27, ulps = 2);
        // Hydrogen mass in daltons - should be equal to hydrogen atomic number
        assert_approx_eq!(f64, hydrogen.data().atomic_mass().value / Mass::new::<dalton>(1.0).value, 1.0078, ulps = 2);

        let helium = Element::Helium(HeliumIsotope::Helium);
        // Helium mass in kg
        assert_approx_eq!(f64, helium.data().atomic_mass().value, 6.646437378005203e-27, ulps = 2);
        // Helium mass in daltons - should be equal to hydrogen atomic number
        assert_approx_eq!(f64, helium.data().atomic_mass().value / Mass::new::<dalton>(1.0).value, 4.0026, ulps = 2);

        let oxygen = Element::Oxygen(OxygenIsotope::Oxygen);
        // Mass in kg
        assert_approx_eq!(f64, oxygen.data().atomic_mass().value, 2.656001129702079e-26, ulps = 2);
        // Mass in daltons - should be equal to atomic number
        assert_approx_eq!(f64, oxygen.data().atomic_mass().value / Mass::new::<dalton>(1.0).value, 15.994899999999998, ulps = 2);
    }

    #[test]
    fn test_atomic_mass_of_isotopes() {
        let deuterium = Element::Hydrogen(HydrogenIsotope::Deuterium);
        // Deuterium mass in kg
        assert_approx_eq!(f64, deuterium.data().atomic_mass().value, 3.344473473002618e-27, ulps = 2);
        // Deuterium mass in daltons - should be equal to deuterium atomic number
        assert_approx_eq!(f64, deuterium.data().atomic_mass().value / Mass::new::<dalton>(1.0).value, 2.0141, ulps = 2);

        let tritium = Element::Hydrogen(HydrogenIsotope::Tritium);
        // Deuterium mass in kg
        assert_approx_eq!(f64, tritium.data().atomic_mass().value, 5.008158480003921e-27, ulps = 2);
        // Deuterium mass in daltons - should be equal to deuterium atomic number
        assert_approx_eq!(f64, tritium.data().atomic_mass().value / Mass::new::<dalton>(1.0).value, 3.016, ulps = 2);
    }

    #[test]
    fn test_molecular_weight() {
        // let molecular_hydrogen = Molecule::new(vec!((Element::Hydrogen(1), 2)));
        // let carbon_monoxide = Molecule::new(vec!((Element::Carbon(1), 1), (Element::Oxygen(1), 1)));
        //
        // println!("molecular H {:?}", molecular_hydrogen.molecular_weight());
        // println!("CO {:?}", carbon_monoxide.molecular_weight());
    }
}
