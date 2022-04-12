#![allow(dead_code)]

use crate::chemistry::elements::elements::Element;
use crate::chemistry::elements::silicon::SiliconIsotope;
use crate::chemistry::molecules::molecules::Molecule;
use crate::cloud::cloud::{CloudOptions, MolecularCloud};
use crate::coordinates::coordinates::{Cartesian, Coordinates};
use crate::formulae::constants::{
    GRAVITATIONAL_CONSTANT, GRAVITATIONAL_CONSTANT_COLLPASE_ADJUSTMENT,
};
use crate::formulae::formulae::{time, volume};
use crate::gas::gas::{Composition, UniformGas};
use crate::transition::transition::Interpolatable;
use crate::units::units::mass::solar_mass;
use itertools::Itertools;
use num::Float;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_seeder::Seeder;
use std::time::Instant;
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::f64::*;
use uom::si::length::{light_year, parsec};
use uom::si::mass::kilogram;
use uom::si::mass_density::gigagram_per_cubic_meter;
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::time::{second, year};

#[macro_use]
extern crate uom;
extern crate float_cmp;
extern crate lazy_static;

mod chemistry;
mod cloud;
mod coordinates;
mod formulae;
mod gas;
mod hash;
mod transition;
mod units;
mod wavelength;

pub fn main() {
    // let volume = volume::sphere_volume_from_length(Length::new::<light_year>(600.0));
    // let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(15.0);
    // let cloud = UniformGas::composite_from_vacuum_properties(
    //     volume,
    //     200000000.0,
    //     temperature,
    //     Composition(vec![
    //         (Molecule::molecular_hydrogen(), 84.0),
    //         (Molecule::carbon_monoxide(), 10.0),
    //         (Molecule::atomic_helium(), 5.0),
    //         (
    //             Molecule::new(vec![(Element::Silicon(SiliconIsotope::Silicon), 1)]),
    //             1.0,
    //         ),
    //     ]),
    // );

    // println!("will collapse? {}", cloud.stable());

    // let n = cloud.next_state();

    // println!("{:?}", cloud);
    // println!("{:?}", n.0);
    // println!("{:?}", n.1.into_format_args(year, Abbreviation));

    let volume = volume::sphere_volume_from_length(Length::new::<parsec>(2.0));
    let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(50.0);
    let cloud = UniformGas::composite_from_vacuum_properties(
        volume,
        4.0e2 * 1000000.0,
        temperature,
        Composition(vec![
            (Molecule::molecular_hydrogen(), 95.0),
            (Molecule::carbon_monoxide(), 2.5),
            (Molecule::atomic_helium(), 2.4),
            (
                Molecule::new(vec![(Element::Silicon(SiliconIsotope::Silicon), 1)]),
                0.1,
            ),
        ]),
    );

    // let n = cloud.next_state();

    println!(
        "jeans radius? {}",
        cloud.jeans_radius().into_format_args(parsec, Abbreviation)
    );
    println!(
        "jeans mass? {:?}",
        cloud
            .jeans_mass()
            .into_format_args(solar_mass, Abbreviation)
    );
    println!("potential? {:?}", cloud.potential_energy());
    // println!("next? {}", n.1.into_format_args(year, Abbreviation));

    // println!("tff {:?}", tff.into_format_args(year, Abbreviation));
    // let now = Instant::now();
    // let volume = volume::sphere_volume_from_length(Length::new::<light_year>(600.0));
    // let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(15.0);
    // let cloud = UniformGas::composite_from_vacuum_properties(
    //     volume,
    //     300.0,
    //     temperature,
    //     Composition(vec![
    //         (Molecule::molecular_hydrogen(), 84.0),
    //         (Molecule::carbon_monoxide(), 10.0),
    //         (Molecule::atomic_helium(), 5.0),
    //         (
    //             Molecule::new(vec![(Element::Silicon(SiliconIsotope::Silicon), 1)]),
    //             1.0,
    //         ),
    //     ]),
    // );
    // let elapsed = now.elapsed();

    // println!(
    //     "mass: {:?}",
    //     cloud.mass.into_format_args(solar_mass, Abbreviation)
    // );
    // println!("stable: {:?}", cloud.stable());

    // println!("took {:?}micros", elapsed.as_micros());
    // println!("took {:?}ms", elapsed.as_millis());
    // println!("took {:?}nanos", elapsed.as_nanos());
}
