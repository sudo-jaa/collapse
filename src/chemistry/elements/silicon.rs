use uom::si::f64::Time;
use uom::si::time::{second, yoctosecond};
use crate::chemistry::elements::elements::{DecayType, DecayProperties, Isotope, Stability, WithNucleonEmission, DecayProcess, Beta};
use crate::chemistry::elements::elements::Stability::{Stable, Unstable};
use crate::chemistry::elements::hydrogen::HydrogenIsotope;

#[derive(Debug)]
pub enum SiliconIsotope {
    Silicon22,
    Silicon23,
    Silicon24,
    Silicon25,
    Silicon26,
    Silicon27,
    Silicon,
    Silicon29,
    Silicon30,
    Silicon31,
    Silicon32,
    Silicon33,
    Silicon34,
    Silicon35,
    Silicon36,
    Silicon37,
    Silicon38,
    Silicon39,
    Silicon40,
    Silicon41,
    Silicon42,
    Silicon43,
    Silicon44,
}

impl Isotope for SiliconIsotope {
    fn atomic_mass_number(&self) -> f64 {
        match self {
            SiliconIsotope::Silicon22 => 22.0357,
            SiliconIsotope::Silicon23 => 23.0254,
            SiliconIsotope::Silicon24 => 24.0115,
            SiliconIsotope::Silicon25 => 25.0041,
            SiliconIsotope::Silicon26 => 25.9923,
            SiliconIsotope::Silicon27 => 26.9867,
            SiliconIsotope::Silicon => 27.9769,
            SiliconIsotope::Silicon29 => 28.9764,
            SiliconIsotope::Silicon30 => 29.9737,
            SiliconIsotope::Silicon31 => 30.9753,
            SiliconIsotope::Silicon32 => 31.9741,
            SiliconIsotope::Silicon33 => 32.9779,
            SiliconIsotope::Silicon34 => 33.9785,
            SiliconIsotope::Silicon35 => 34.9845,
            SiliconIsotope::Silicon36 => 35.9866,
            SiliconIsotope::Silicon37 => 36.9929,
            SiliconIsotope::Silicon38 => 37.9955,
            SiliconIsotope::Silicon39 => 39.0024,
            SiliconIsotope::Silicon40 => 40.0058,
            SiliconIsotope::Silicon41 => 41.0130,
            SiliconIsotope::Silicon42 => 42.0176,
            SiliconIsotope::Silicon43 => 43.0248,
            SiliconIsotope::Silicon44 => 44.0306
        }
    }

    fn stability(&self) -> Stability {
        match self {
            SiliconIsotope::Silicon22 => todo!(),
            SiliconIsotope::Silicon23 => todo!(),
            SiliconIsotope::Silicon24 => todo!(),
            SiliconIsotope::Silicon25 => todo!(),
            SiliconIsotope::Silicon26 => todo!(),
            SiliconIsotope::Silicon27 => todo!(),
            SiliconIsotope::Silicon => Stable,
            SiliconIsotope::Silicon29 => Stable,
            SiliconIsotope::Silicon30 => Stable,
            SiliconIsotope::Silicon31 => todo!(),
            SiliconIsotope::Silicon32 => todo!(),
            SiliconIsotope::Silicon33 => todo!(),
            SiliconIsotope::Silicon34 => todo!(),
            SiliconIsotope::Silicon35 => todo!(),
            SiliconIsotope::Silicon36 => todo!(),
            SiliconIsotope::Silicon37 => todo!(),
            SiliconIsotope::Silicon38 => todo!(),
            SiliconIsotope::Silicon39 => todo!(),
            SiliconIsotope::Silicon40 => todo!(),
            SiliconIsotope::Silicon41 => todo!(),
            SiliconIsotope::Silicon42 => todo!(),
            SiliconIsotope::Silicon43 => todo!(),
            SiliconIsotope::Silicon44 => todo!(),
        }
    }
}
