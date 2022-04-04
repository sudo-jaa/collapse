use rand::Rng;
use rand_chacha::ChaCha20Rng;
use rand_seeder::rand_core::SeedableRng;
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::amount_of_substance::mole;
use uom::si::f64::{AmountOfSubstance, Length, Mass, MassDensity, MolarMass, Pressure, ThermodynamicTemperature, Volume};
use uom::si::length::{astronomical_unit, light_year};
use uom::si::mass::kilogram;
use uom::si::power::watt;
use uom::si::pressure::pascal;
use uom::si::volume::cubic_meter;
use crate::{Cartesian, Coordinates, solar_mass};
use crate::chemistry::molecules::molecules::Molecule;
use crate::formulae::formulae::{length, mass, volume};
use crate::units::units::volume::cubic_lightyear;

impl Cartesian for MolecularCloud {
    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub struct CloudOptions {
    pub use_randomness: bool,
    pub core_formation_chance: f64,
    pub core_formation_dropoff: f64,
}

#[derive(Debug)]
pub struct MolecularCloud {
    pub volume: Volume,
    pub radius: Length,

    pub coordinates: Coordinates,

    pub average_density: MassDensity,
    pub mass: Mass,

    pub contents: Vec<(Molecule, f64)>,
}

impl MolecularCloud {
    pub fn pressure() -> Pressure {
        Pressure::new::<pascal>(1.322e-11)
    }

    pub fn new(coordinates: Coordinates, radius: Length, average_density: MassDensity, contents: Vec<(Molecule, f64)>, options: CloudOptions) -> MolecularCloud {
        let mut rng = ChaCha20Rng::seed_from_u64(coordinates.hash);

        let (volume, actual_radius) = {
            if !options.use_randomness {
                (volume::sphere_volume_from_length(radius), radius)
            } else {
                let modifier = rng.gen_range(0.5..1.5);
                let volume = volume::sphere_volume_from_length(radius * modifier);
                (volume, length::sphere_radius_from_volume(volume))
            }
        };

        /// The mass of the diffuse part of the molecular cloud.
        // get the diffuse mass level by getting the masses of each constituent material and summing them.
        let diffuse_mass = contents.iter().map(|(material, ratio)| {
            println!();
            let volume_of_material: Volume = volume * (*ratio / 100.0);
            mass::from_volume_and_density(volume_of_material, average_density)
        }).fold(Mass::new::<kilogram>(0.0), |acc, mass| acc + mass);

        println!("diffuse mass {:?}", diffuse_mass.into_format_args(solar_mass, Abbreviation));

        // use the diameter to determine number of available cores.
        // for example a cloud 1 lightyear across can have a maximum of 1 possible core.
        // A cloud 50 light years across has 200 possible cores
        let number_of_possible_cores = (actual_radius.value * 2.0 / Length::new::<light_year>(1.0).value) * 4.0;
        println!();
        println!("possible cores {}", number_of_possible_cores);

        // the number of actual cores in this cloud
        let mut chance = options.core_formation_chance;
        let possible_cores = (0..(number_of_possible_cores as usize)).fold(0, |acc, _| {
            let roll = rng.gen_range(0.0..100.0);
            if roll <= chance {
                chance *= options.core_formation_dropoff;
                acc + 1
            } else {
                acc + 0
            }
        });

        println!("cores: {}", possible_cores);

        // let core_density: MassDensity = {
        //     let value = 10e8;
        //     // let value = rng.gen_range(10e4..10e6);
        //     // MassDensity::new::<hydrogen_atom_per_cubic_centimeter>(value)
        // };

        // println!("diffuse density: {:?}", average_density.into_format_args(hydrogen_atom_per_cubic_centimeter, Abbreviation));
        // println!("core density: {:?}", core_density.into_format_args(hydrogen_atom_per_cubic_centimeter, Abbreviation));
        // println!("average core mass {:?}", mass::from_volume_and_density(Volume::new::<cubic_lightyear>(1.0), core_density).into_format_args(solar_mass, Abbreviation));

        println!();
        MolecularCloud {
            radius: actual_radius,
            volume,
            coordinates,
            average_density,
            mass: diffuse_mass,
            contents
        }
    }
}



