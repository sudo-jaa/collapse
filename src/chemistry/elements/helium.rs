use uom::si::f64::Time;
use uom::si::time::{second, yoctosecond};
use crate::chemistry::elements::elements::{DecayType, DecayProperties, Isotope, Stability, WithNucleonEmission, DecayProcess, Beta};
use crate::chemistry::elements::elements::Stability::{Stable, Unstable};
use crate::chemistry::elements::hydrogen::HydrogenIsotope;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum HeliumIsotope {
    Helium,
    Helium2,
    Helium3,
    Helium5,
    Helium6,
    Helium7,
    Helium8,
    Helium9,
    Helium10
}

impl Isotope for HeliumIsotope {
    fn atomic_mass_number(&self) -> f64 {
        match self {
            HeliumIsotope::Helium => 4.0026,
            HeliumIsotope::Helium3 => 3.0160,
            HeliumIsotope::Helium2 => 2.0158,
            HeliumIsotope::Helium5 => 5.0120,
            HeliumIsotope::Helium6 => 6.0188,
            HeliumIsotope::Helium7 => 7.0279,
            HeliumIsotope::Helium8 => 8.0339,
            HeliumIsotope::Helium9 => 9.0439,
            HeliumIsotope::Helium10 => 10.0528,
        }
    }

    fn stability(&self) -> Stability {
        match self {
            HeliumIsotope::Helium => Stable,
            HeliumIsotope::Helium2 => Unstable(DecayProperties {
                half_life: Time::new::<second>(10e-9),
                decay_process: vec!(
                    DecayProcess {
                        chance: 99.99,
                        mode: DecayType::WithNucleonEmission(WithNucleonEmission::ProtonEmission),
                        child: (2, Box::new(HydrogenIsotope::Hydrogen))
                    },
                    DecayProcess {
                        chance: 0.01,
                        mode: DecayType::Beta(Beta::BetaPlus),
                        child: (1, Box::new(HydrogenIsotope::Deuterium))
                    }
                )
            }),
            HeliumIsotope::Helium3 => Stable,
            HeliumIsotope::Helium5 => Unstable(DecayProperties {
                half_life: Time::new::<yoctosecond>(602.0),
                decay_process: vec!(
                    DecayProcess {
                        chance: 100.0,
                        mode: DecayType::WithNucleonEmission(WithNucleonEmission::NeutronEmission),
                        child: (1, Box::new(HeliumIsotope::Helium))
                    }
                )
            }),
            HeliumIsotope::Helium6 => todo!(),
            HeliumIsotope::Helium7 => todo!(),
            HeliumIsotope::Helium8 => todo!(),
            HeliumIsotope::Helium9 => todo!(),
            HeliumIsotope::Helium10 => todo!(),
        }
    }
}
