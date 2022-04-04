use uom::si::f64::{Mass, Time};
use crate::chemistry::elements::carbon::CarbonIsotope;
use crate::chemistry::elements::helium::HeliumIsotope;
use crate::chemistry::elements::hydrogen::HydrogenIsotope;
use crate::chemistry::elements::oxygen::OxygenIsotope;
use crate::chemistry::elements::silicon::SiliconIsotope;
use crate::units::units::mass::dalton;

pub enum Stability {
    Stable,
    Unstable(DecayProperties)
}

pub struct DecayProperties {
    pub(crate) half_life: Time,
    pub(crate) decay_process: Vec<DecayProcess>
}

pub struct DecayProcess {
    pub(crate) chance: f64,
    pub(crate) mode: DecayType,
    pub(crate) child: (usize, Box<dyn Isotope>)
}

pub enum DecayType {
    WithNucleonEmission(WithNucleonEmission),
    Beta(Beta)
}

pub enum WithNucleonEmission {
    AlphaDecay,
    ProtonEmission,
    DoubleProtonEmission,
    NeutronEmission,
    DoubleNeutronEmission,
    TripleNeutronEmission,
    SpontaneousFission,
    ClusterDecay
}

pub enum Beta {
    BetaMinus,
    BetaPlus,
    ElectronCapture,
    BoundState,
    Double,
    DoubleElectronCapture,
    ElectronCaptureWithProtonEmission,
    DoublePositron
}

pub trait Isotope {
    fn atomic_mass_number(&self) -> f64;
    fn stability(&self) -> Stability;
    fn atomic_mass(&self) -> Mass {
        Mass::new::<dalton>(self.atomic_mass_number())
    }
}

#[derive(Debug)]
pub enum Element {
    Hydrogen(HydrogenIsotope),
    Helium(HeliumIsotope),
    Oxygen(OxygenIsotope),
    Carbon(CarbonIsotope),
    Silicon(SiliconIsotope)
}

impl Element {
    pub fn data(&self) -> &dyn Isotope {
       match self {
           Element::Hydrogen(element) => element,
           Element::Helium(element) => element,
           Element::Oxygen(element) => element,
           Element::Carbon(element) => element,
           Element::Silicon(element) => element
       }
    }

    pub fn symbol(&self) -> &str {
        match self {
            Element::Hydrogen(_) => "H",
            Element::Helium(_) => "He",
            Element::Oxygen(_) => "O",
            Element::Carbon(_) => "C",
            Element::Silicon(_) => "Si"
        }
    }
}

