use std::time::Instant;
use uom::si::f64::*;
use uom::si::mass::{kilogram};
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::length::{light_year, parsec};
use uom::si::thermodynamic_temperature::kelvin;
use crate::chemistry::elements::elements::Element;
use crate::chemistry::elements::silicon::SiliconIsotope;
use crate::chemistry::molecules::molecules::Molecule;
use crate::cloud::cloud::{CloudOptions, MolecularCloud};
use crate::coordinates::coordinates::{Cartesian, Coordinates};
use crate::formulae::formulae::volume;
use crate::gas::gas::{Composition, Gas};
use crate::units::units::mass::{solar_mass};

#[macro_use]
extern crate uom;
extern crate float_cmp;
extern crate lazy_static;

mod units;
mod formulae;
mod coordinates;
mod hash;
mod cloud;
mod chemistry;
mod gas;
mod transition;

pub fn main() {
    let now = Instant::now();
    let volume = volume::sphere_volume_from_length(Length::new::<light_year>(600.0));
    let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(15.0);
    let cloud = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, Composition(vec!(
        (Molecule::molecular_hydrogen(), 84.0),
        (Molecule::carbon_monoxide(), 10.0),
        (Molecule::atomic_helium(), 5.0),
        (Molecule::new(
            vec!(
                (Element::Silicon(SiliconIsotope::Silicon), 1)
            )
        ), 1.0)
    )));
    let elapsed = now.elapsed();

    println!("mass: {:?}", cloud.mass.into_format_args(solar_mass, Abbreviation));
    println!("stable: {:?}", cloud.stable());

    println!("took {:?}micros", elapsed.as_micros());
    println!("took {:?}ms", elapsed.as_millis());
    println!("took {:?}nanos", elapsed.as_nanos());
}
