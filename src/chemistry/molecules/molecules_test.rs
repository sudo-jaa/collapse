#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use uom::fmt::DisplayStyle::Abbreviation;
    use uom::si::f64::{AmountOfSubstance, Mass, MolarMass, Volume};
    use uom::si::mass::gram;
    use uom::si::molar_mass::gram_per_mole;
    use uom::si::power::watt;
    use uom::si::volume::cubic_meter;
    use uom::si::amount_of_substance::mole;
    use crate::chemistry::elements::carbon::CarbonIsotope;
    use crate::units::units::mass::dalton;
    use crate::chemistry::elements::elements::Element;
    use crate::chemistry::elements::helium::HeliumIsotope;
    use crate::chemistry::elements::hydrogen::HydrogenIsotope;
    use crate::chemistry::elements::oxygen::OxygenIsotope;
    use crate::chemistry::molecules::molecules::Molecule;

    #[test]
    fn test_molecular_weight() {
        let molecular_hydrogen = Molecule::molecular_hydrogen();
        assert_eq!(molecular_hydrogen.molecular_weight(), Mass::new::<dalton>(2.0156));
        assert_eq!(molecular_hydrogen.molar_mass(), MolarMass::new::<gram_per_mole>(2.0156));

        let carbon_monoxide = Molecule::carbon_monoxide();
        assert_eq!(carbon_monoxide.molecular_weight(), Mass::new::<dalton>(27.9949));
        assert_eq!(carbon_monoxide.molar_mass(), MolarMass::new::<gram_per_mole>(27.9949));

        let water = Molecule::water();
        assert_eq!(water.molecular_weight(), Mass::new::<dalton>(18.0105));
        assert_eq!(water.molar_mass(), MolarMass::new::<gram_per_mole>(18.0105));
    }

    #[test]
    fn amount_of_substance_test() {
        // https://www.bbc.co.uk/bitesize/guides/z84wfrd/revision/3
        let carbon_dioxide = Molecule::carbon_dioxide();

        let moles_in_22g = carbon_dioxide.amount(Mass::new::<gram>(22.0));
        assert_eq!(moles_in_22g, AmountOfSubstance::new::<mole>(0.5001159359669741));

        let mass_in_2_moles = carbon_dioxide.mass_in_amount(AmountOfSubstance::new::<mole>(2.0));
        assert_eq!(mass_in_2_moles, Mass::new::<gram>(87.9796));
    }
}
