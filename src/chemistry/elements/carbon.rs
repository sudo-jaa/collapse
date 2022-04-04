use crate::chemistry::elements::elements::{Isotope, Stability};
use crate::chemistry::elements::elements::Stability::Stable;

#[derive(Debug)]
pub enum CarbonIsotope {
    Carbon8,
    Carbon9,
    Carbon10,
    Carbon11,
    Carbon,
    Caron13,
    Carbon14,
    Carbon15,
    Carbon16,
    Carbon17,
    Carbon18,
    Carbon19,
    Carbon20,
    Carbon21,
    Carbon22,
}

impl Isotope for CarbonIsotope {
    fn atomic_mass_number(&self) -> f64 {
        match self {
            CarbonIsotope::Carbon8 => 8.0376,
            CarbonIsotope::Carbon9 => 9.0310,
            CarbonIsotope::Carbon10 => 10.0168,
            CarbonIsotope::Carbon11 => 11.0114,
            CarbonIsotope::Carbon => 12.0,
            CarbonIsotope::Caron13 => 13.0035,
            CarbonIsotope::Carbon14 => 14.0032,
            CarbonIsotope::Carbon15 => 15.0105,
            CarbonIsotope::Carbon16 => 16.0147,
            CarbonIsotope::Carbon17 => 17.0225,
            CarbonIsotope::Carbon18 => 18.0267,
            CarbonIsotope::Carbon19 => 19.0348,
            CarbonIsotope::Carbon20 => 20.0402,
            CarbonIsotope::Carbon21 => 21.0490,
            CarbonIsotope::Carbon22 => 22.0575
        }
    }

    fn stability(&self) -> Stability {
        match self {
            CarbonIsotope::Carbon8 => todo!(),
            CarbonIsotope::Carbon9 => todo!(),
            CarbonIsotope::Carbon10 => todo!(),
            CarbonIsotope::Carbon11 => todo!(),
            CarbonIsotope::Carbon => Stable,
            CarbonIsotope::Caron13 => Stable,
            CarbonIsotope::Carbon14 => todo!(),
            CarbonIsotope::Carbon15 => todo!(),
            CarbonIsotope::Carbon16 => todo!(),
            CarbonIsotope::Carbon17 => todo!(),
            CarbonIsotope::Carbon18 => todo!(),
            CarbonIsotope::Carbon19 => todo!(),
            CarbonIsotope::Carbon20 => todo!(),
            CarbonIsotope::Carbon21 => todo!(),
            CarbonIsotope::Carbon22 => todo!(),
        }
    }
}
