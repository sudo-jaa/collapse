use uom::si::f64::Time;
use uom::si::time::{year, yoctosecond};
use crate::chemistry::elements::elements::{Beta, DecayType, DecayProperties, Isotope, Stability, WithNucleonEmission, DecayProcess};
use crate::chemistry::elements::elements::Stability::{Stable, Unstable};
use crate::chemistry::elements::helium::HeliumIsotope;

#[derive(Debug, Clone)]
pub enum HydrogenIsotope {
    Hydrogen,
    Deuterium,
    Tritium,
    Hydrogen4,
    Hydrogen5,
    Hydrogen6,
    Hydrogen7
}

impl Isotope for HydrogenIsotope {
    fn atomic_mass_number(&self) -> f64 {
        match self {
            HydrogenIsotope::Hydrogen => 1.0078,
            HydrogenIsotope::Deuterium => 2.0141,
            HydrogenIsotope::Tritium => 3.0160,
            HydrogenIsotope::Hydrogen4 => 4.0264,
            HydrogenIsotope::Hydrogen5 => 5.0351,
            HydrogenIsotope::Hydrogen6 => 6.0449,
            HydrogenIsotope::Hydrogen7 => 7.0527
        }
    }

    fn stability(&self) -> Stability {
        match self {
            HydrogenIsotope::Hydrogen => Stable,
            HydrogenIsotope::Deuterium => Stable,
            HydrogenIsotope::Tritium => Unstable(DecayProperties {
                half_life: Time::new::<year>(12.322),
                decay_process: vec!(
                    DecayProcess {
                        chance: 100.0,
                        mode: DecayType::Beta(Beta::BetaMinus),
                        child: (1, Box::new(HeliumIsotope::Helium3))
                    }
                )
            }),
            HydrogenIsotope::Hydrogen4 => Unstable(DecayProperties {
                half_life: Time::new::<yoctosecond>(139.0),
                decay_process: vec!(
                    DecayProcess {
                        chance: 100.0,
                        mode: DecayType::WithNucleonEmission(WithNucleonEmission::NeutronEmission),
                        child: (1, Box::new(HydrogenIsotope::Tritium))
                    }
                )
            }),
            HydrogenIsotope::Hydrogen5 => Unstable(DecayProperties {
                half_life: Time::new::<yoctosecond>(86.0),
                decay_process: vec!(
                    DecayProcess {
                        chance: 100.0,
                        mode: DecayType::WithNucleonEmission(WithNucleonEmission::DoubleNeutronEmission),
                        child: (1, Box::new(HydrogenIsotope::Tritium))
                    }
                )
            }),
            HydrogenIsotope::Hydrogen6 => Unstable(DecayProperties {
                half_life: Time::new::<yoctosecond>(294.0),
                decay_process: vec!(
                    DecayProcess {
                        chance: 50.0,
                        mode: DecayType::WithNucleonEmission(WithNucleonEmission::NeutronEmission),
                        child: (1, Box::new(HydrogenIsotope::Hydrogen5))
                    },
                    DecayProcess {
                        chance: 50.0,
                        mode: DecayType::WithNucleonEmission(WithNucleonEmission::TripleNeutronEmission),
                        child: (1, Box::new(HydrogenIsotope::Tritium))
                    }
                )
            }),
            HydrogenIsotope::Hydrogen7 => Unstable(DecayProperties {
                half_life: Time::new::<yoctosecond>(652.0),
                decay_process: vec!(
                    DecayProcess {
                        chance: 100.0,
                        mode: DecayType::WithNucleonEmission(WithNucleonEmission::DoubleNeutronEmission),
                        child: (1, Box::new(HydrogenIsotope::Hydrogen5))
                    }
                )
            })
        }
    }
}
