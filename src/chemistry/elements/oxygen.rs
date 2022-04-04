use crate::chemistry::elements::elements::{Isotope, Stability};
use crate::chemistry::elements::elements::Stability::Stable;

#[derive(Debug)]
pub enum OxygenIsotope {
   Oxygen11,
   Oxygen12,
   Oxygen13,
   Oxygen14,
   Oxygen15,
   Oxygen,
   Oxygen17,
   Oxygen18,
   Oxygen19,
   Oxygen20,
   Oxygen21,
   Oxygen22,
   Oxygen23,
   Oxygen24,
   Oxygen25,
   Oxygen26,
   Oxygen27,
   Oxygen28,
}

impl Isotope for OxygenIsotope {
   fn atomic_mass_number(&self) -> f64 {
      match self {
         OxygenIsotope::Oxygen11 => 11.0512,
         OxygenIsotope::Oxygen12 => 12.0343,
         OxygenIsotope::Oxygen13 => 13.0248,
         OxygenIsotope::Oxygen14 => 14.0085,
         OxygenIsotope::Oxygen15 => 15.0030,
         OxygenIsotope::Oxygen => 15.9949,
         OxygenIsotope::Oxygen17 => 16.9991,
         OxygenIsotope::Oxygen18 => 17.9991,
         OxygenIsotope::Oxygen19 => 19.0035,
         OxygenIsotope::Oxygen20 => 20.0040,
         OxygenIsotope::Oxygen21 => 21.0086,
         OxygenIsotope::Oxygen22 => 22.0099,
         OxygenIsotope::Oxygen23 => 23.0157,
         OxygenIsotope::Oxygen24 => 24.0198,
         OxygenIsotope::Oxygen25 => 25.0293,
         OxygenIsotope::Oxygen26 => 26.0372,
         OxygenIsotope::Oxygen27 => 27.0479,
         OxygenIsotope::Oxygen28 => 28.0559,
      }
   }

   fn stability(&self) -> Stability {
      match self {
         OxygenIsotope::Oxygen11 => todo!(),
         OxygenIsotope::Oxygen12 => todo!(),
         OxygenIsotope::Oxygen13 => todo!(),
         OxygenIsotope::Oxygen14 => todo!(),
         OxygenIsotope::Oxygen15 => todo!(),
         OxygenIsotope::Oxygen => Stable,
         OxygenIsotope::Oxygen17 => Stable,
         OxygenIsotope::Oxygen18 => Stable,
         OxygenIsotope::Oxygen19 => todo!(),
         OxygenIsotope::Oxygen20 => todo!(),
         OxygenIsotope::Oxygen21 => todo!(),
         OxygenIsotope::Oxygen22 => todo!(),
         OxygenIsotope::Oxygen23 => todo!(),
         OxygenIsotope::Oxygen24 => todo!(),
         OxygenIsotope::Oxygen25 => todo!(),
         OxygenIsotope::Oxygen26 => todo!(),
         OxygenIsotope::Oxygen27 => todo!(),
         OxygenIsotope::Oxygen28 => todo!(),
      }
   }
}
