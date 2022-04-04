use std::fmt::{Display, Formatter};
use uom::si::amount_of_substance::mole;
use uom::si::f64::{AmountOfSubstance, Mass, MolarMass};
use uom::si::mass::gram;
use uom::si::molar_mass::gram_per_mole;
use crate::chemistry::elements::carbon::CarbonIsotope;
use crate::chemistry::elements::elements::Element;
use crate::chemistry::elements::helium::HeliumIsotope;
use crate::chemistry::elements::hydrogen::HydrogenIsotope;
use crate::chemistry::elements::oxygen::OxygenIsotope;
use crate::units::units::mass::dalton;

/// A component of a molecular formula. Represents the actual elemental isotope
/// and the number of occurrences in the molecule.
type MolecularComponent = (Element, usize);

#[derive(Debug, Clone)]
pub struct Molecule(Vec<MolecularComponent>);

impl Molecule {
    /// Create a new molecule from a provided vector of molecular components
    pub fn new(elements: Vec<MolecularComponent>) -> Molecule {
        Molecule(elements)
    }

    /// Helper function for creating an instance of a molecular hydrogen molecule
    pub fn molecular_hydrogen() -> Molecule {
        Molecule::new(vec!(
            (Element::Hydrogen(HydrogenIsotope::Hydrogen), 2)
        ))
    }

    /// Helper function for creating an instance of a atomic hydrogen molecule
    pub fn atomic_hydrogen() -> Molecule {
        Molecule::new(vec!(
            (Element::Hydrogen(HydrogenIsotope::Hydrogen), 1)
        ))
    }


    /// Helper function for creating an instance of a water molecule
    pub fn water() -> Molecule {
        Molecule::new(
            vec!(
                (Element::Hydrogen(HydrogenIsotope::Hydrogen), 2),
                (Element::Oxygen(OxygenIsotope::Oxygen), 1)
            )
        )
    }

    /// Helper function for creating an instance of a carbon monoxide molecule
    pub fn carbon_monoxide() -> Molecule {
        Molecule::new(vec!(
            (Element::Carbon(CarbonIsotope::Carbon), 1),
            (Element::Oxygen(OxygenIsotope::Oxygen), 1)
        ))
    }

    /// Helper function for creating an instance of a carbon dioxide molecule
    pub fn carbon_dioxide() -> Molecule {
        Molecule::new(vec!(
            (Element::Carbon(CarbonIsotope::Carbon), 1),
            (Element::Oxygen(OxygenIsotope::Oxygen), 2)
        ))
    }

    /// Helper function for creating an instance of a O2 molecule
    pub fn molecular_oxygen() -> Molecule {
        Molecule::new(vec!(
            (Element::Oxygen(OxygenIsotope::Oxygen), 2)
        ))
    }

    /// Helper function for creating an instance of an atomic helium molecule
    pub fn atomic_helium() -> Molecule {
        Molecule::new(vec!(
            (Element::Helium(HeliumIsotope::Helium), 1)
        ))
    }

    fn relative_formula_mass(&self) -> f64 {
        self.0.iter().fold(0.0, |acc, (element, count)| {
            acc + (element.data().atomic_mass_number() * *count as f64)
        })
    }

    /// Get the molecular weight of a given molecule
    pub fn molecular_weight(&self) -> Mass {
        Mass::new::<dalton>(self.relative_formula_mass())
    }

    /// Get the molecular mass of a given molecule in moles
    pub fn molar_mass(&self) -> MolarMass {
        MolarMass::new::<gram_per_mole>(self.relative_formula_mass())
    }

    /// Get the amount of substance in a given mass
    pub fn amount(&self, mass: Mass) -> AmountOfSubstance {
        AmountOfSubstance::new::<mole>((mass.value / Mass::new::<gram>(1.0).value) / self.relative_formula_mass())
    }

    /// Get the mass of an amount of substance
    pub fn mass_in_amount(&self, amount: AmountOfSubstance) -> Mass {
        Mass::new::<gram>(amount.value * self.relative_formula_mass())
    }
}

impl Display for Molecule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formula = self.0.iter().fold(String::new(), |formula, molecule| {
            format!("{}{}{}", formula, molecule.0.symbol(), if molecule.1 > 1 {molecule.1.to_string()} else {String::new()})
        });
        write!(f, "{}", formula)
    }
}
