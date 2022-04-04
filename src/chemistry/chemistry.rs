use uom::si::f64::Mass;
use crate::units::units::mass::dalton;

use lazy_static::lazy_static;


// A new element with a given isotop if applicable.
// An isotope value of one is the base isotope of the given element.
// #[derive(Debug)]
// pub enum Element {
//     Hydrogen(Isotope),
//     Helium(Isotope),
//     Oxygen(Isotope),
//     Carbon(Isotope)
// }

// impl Element {
//     /// Get the mass in kg of an element, taking into account it's isotope
//    pub fn mass(&self) -> Mass {
//         let (atomic_mass, nucleons, isotope) = match self {
//             Element::Hydrogen(iso) => (1.0078, 1, iso - 1),
//             Element::Helium(iso) => (4.0026, 4, iso - 1),
//             Element::Oxygen(iso) => (15.999, 12, iso - 1),
//             Element::Carbon(iso) => (12.011, iso - 1)
//         };
//         let neutron_mass = Mass::new::<dalton>(1.007);
//         Mass::new::<dalton>(atomic_mass) + (neutron_mass * isotope as f64)
//     }
// }

// pub struct Molecule(Vec<(usize, dyn Element)>);
//
// impl Molecule {
//     pub fn new(elements: Vec<(usize, dyn Element)>) -> Molecule {
//         Molecule(elements)
//     }
//
//     /// Get the molecular weight of a given molecule
//     pub fn molecular_weight(&self) -> Mass {
//         self.0.iter().fold(Mass::new::<dalton>(0.0), |acc, (count, element)| {
//             acc + (element.mass() * *count as f64)
//         })
//     }
// }
