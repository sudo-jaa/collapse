use crate::chemistry::molecules::molecules::Molecule;
use crate::formulae::constants::{AVOGADRO_CONSTANT, GAS_CONSTANT};
use crate::formulae::formulae::{density, force, length, mass, time, wavelength};
use crate::solar_mass;
use crate::transition::transition::{EasingFunction, Interpolatable, Interpolationf64AsyncOptions};
use crate::units::units::time::{million_year, thousand_year};
use crate::wavelength::wavelength::Wavelength;
use std::collections::HashMap;
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::amount_of_substance::mole;
use uom::si::energy::joule;
use uom::si::f64::{
    AmountOfSubstance, Energy, Force, Frequency, Mass, MassDensity, Pressure,
    ThermodynamicTemperature, Time, Volume,
};
use uom::si::frequency::hertz;
use uom::si::mass::kilogram;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::pressure::pascal;
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::volume::cubic_meter;

#[derive(Debug, Clone)]
enum CosmicState {
    Gas,
    ProtoStar,
    Star,
}

pub trait Gas {}

#[derive(Debug, Clone)]
pub struct Composition(pub Vec<(Molecule, f64)>);

impl Interpolatable for Composition {
    type AO = ();
    fn interpolate(
        &self,
        other: &Self,
        transition: f64,
        ease: Option<EasingFunction>,
        asynchronous_options: Option<Self::AO>,
    ) -> Self {
        // [a: 50.0, b: 30.0], [a: 20.0, b: 20.0, c: 60.0] - target has values that dont exist in origin
        // [a: 20.0, b: 20.0, c: 60.0], [a: 50.0, b: 30.0] - origin has values that dont exist in target
        // [a: 50.0, b: 50.0], [a: 30.0, b: 70.0] - parity between arrays

        let mut origin: HashMap<Molecule, f64> = HashMap::new();
        let mut target: HashMap<Molecule, f64> = HashMap::new();

        self.0.iter().for_each(|(molecule, ratio)| {
            let m = origin.entry(molecule.clone()).or_insert(0.0);
            *m += ratio;
        });

        other.0.iter().for_each(|(molecule, ratio)| {
            let m = target.entry(molecule.clone()).or_insert(0.0);
            *m += ratio;
        });

        // first, fill the origin with empty values for each non-matching target
        other.0.iter().for_each(|(molecule, _)| {
            origin.entry(molecule.clone()).or_insert(0.0);
        });

        // and then do the same for the target map
        self.0.iter().for_each(|(molecule, _)| {
            target.entry(molecule.clone()).or_insert(0.0);
        });

        let mut new_composition = Composition(vec![]);
        origin.into_iter().for_each(|(molecule, ratio)| {
            let target_value = target
                .get(&molecule)
                .unwrap_or_else(|| panic!("cannot unwrap"));
            new_composition.0.push((
                molecule,
                ratio.interpolate(target_value, transition, ease, None),
            ));
        });

        new_composition
    }
}

#[derive(Debug, Clone)]
pub struct UniformGas {
    pub volume: Volume,
    pub pressure: Pressure,
    pub moles: AmountOfSubstance,
    pub temperature: ThermodynamicTemperature,

    pub materials: Composition,

    pub mass: Mass,
    pub density: MassDensity,

    state: CosmicState,
    fragmentations: usize, //TODO!!!! determine the number of fragmentations this cloud can experience during collapse
}

impl UniformGas {
    pub fn next_state(&self) -> (&UniformGas, Time) {
        match self.stable() {
            true => (self, Time::new::<million_year>(1.0)),
            false => {
                let x = 1;
                (self, time::gravitational_freefall(self.density))
            }
        }
    }

    pub fn radiation_wavelength(&self) -> Wavelength {
        wavelength::from_temperature(self.temperature)
    }

    // pub fn resize(self, new_volume: Volume) -> (UniformGas, Time) {
    // collapse is isothermal, so the temperature won't immediately

    // let original_volume = self.volume;
    // let new_density = {
    //     if original_volume == new_volume {
    //         self.density
    //     } else {
    //         if original_volume > new_volume {
    //             // gas has shrunk
    //             let increase = -100.0
    //                 * ((new_volume.value - original_volume.value) / (original_volume.value));
    //             MassDensity::new::<kilogram_per_cubic_meter>(
    //                 self.density.value * (1.0 + (increase / 100.0)),
    //             )
    //         } else {
    //             // gas has grown
    //             let decrease = 100.0
    //                 * ((new_volume.value - original_volume.value) / (original_volume.value));
    //             MassDensity::new::<kilogram_per_cubic_meter>(
    //                 self.density.value * (1.0 - (decrease / 100.0)),
    //             )
    //         }
    //     }
    // };

    // // energy gained from each particle releasing potential energy during collapse
    // let total_energy_gain = {
    //     if original_volume == new_volume {
    //         Energy::new::<joule>(0.0)
    //     } else {
    //         let original_radius = length::sphere_radius_from_volume(original_volume);
    //         let new_radius = length::sphere_radius_from_volume(new_volume);
    //         let average_distance_moved = original_radius - new_radius;
    //         //find force for each molecule type
    //         let total_energy = self.materials.0.iter().fold(
    //             Energy::new::<joule>(0.0),
    //             |acc, (material, ratio)| {
    //                 let mass_of_component =
    //                     Mass::new::<kilogram>((self.mass / 2.0).value * (ratio / 100.0));

    //                 let force_of_single_molecule: Force = force::from_gravitation(
    //                     Mass::new::<kilogram>((self.mass / 2.0).value),
    //                     material.molecular_weight(),
    //                     average_distance_moved,
    //                 );

    //                 let joules_per_meter = Energy::new::<joule>(force_of_single_molecule.value);
    //                 let joule_of_journey = joules_per_meter * average_distance_moved;

    //                 let cumulative_energy: Energy = Energy::new::<joule>(
    //                     (material.amount(mass_of_component).value * AVOGADRO_CONSTANT)
    //                         * joule_of_journey.value,
    //                 );

    //                 acc + cumulative_energy
    //             },
    //         );
    //         total_energy
    //     }
    // };

    // println!("TOTAL ENERGY GAIN FROM COLLAPSE {:?}", total_energy_gain);

    // UniformGas {
    //     volume: new_volume,
    //     pressure: self.pressure, //TODO
    //     moles: self.moles,
    //     temperature: self.temperature, //TODO
    //     materials: self.materials,
    //     mass: self.mass,
    //     density: new_density,
    // }

    // todo!()
    // }

    /// Generate a composite gas entity in a vacuum.
    /// This function generates essentially a gravitationally bound cloud of gas in a vacuum where
    /// the cloud is composed of multiple substances, and is of a generally uniform density described by
    /// the `particles_per_cubic_meter` argument.
    ///
    /// To create a cloud with areas of different densities, instantiate multiple instances of a
    /// gas entity.
    ///
    /// # Arguments
    /// * `volume` - A volume of space in which a cloud occupies
    /// * `particles_per_cubic_meter` - The density of the gas represented as particles per cubic meter of space
    /// * `temperature` - The temperature of the gas cloud
    /// * `materials` - The component materials of the cloud, provided as a vector of tuples that represent the molecule and it's overall ratio of the gas
    ///
    /// # Examples
    ///         let volume = volume::sphere_volume_from_length(Length::new::<light_year>(100.0));
    ///         let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(7.0);
    ///         let cloud = Gas::composite_from_vacuum_properties(volume, 300.0, temperature, vec!(
    ///             (Molecule::molecular_hydrogen(), 84.0),
    ///             (Molecule::carbon_monoxide(), 10.0),
    ///             (Molecule::atomic_helium(), 5.0),
    ///             (Molecule::new(
    ///                 vec!(
    ///                     (Element::Silicon(SiliconIsotope::Silicon), 1)
    ///                 )
    ///             ), 1.0)
    ///         ));
    pub fn composite_from_vacuum_properties(
        volume: Volume,
        particles_per_cubic_meter: f64,
        temperature: ThermodynamicTemperature,
        materials: Composition,
    ) -> UniformGas {
        let density: MassDensity =
            UniformGas::generate_composite_massdensity(&materials, particles_per_cubic_meter);
        let mass = mass::from_volume_and_density(volume, density);
        let moles: AmountOfSubstance = materials.0.iter().fold(
            AmountOfSubstance::new::<mole>(0.0),
            |acc, (material, ratio)| {
                acc + AmountOfSubstance::new::<mole>(
                    (mass.value / material.molar_mass().value) * (ratio / 100.0),
                )
            },
        );
        UniformGas {
            volume,
            pressure: Pressure::new::<pascal>(
                (moles.value * GAS_CONSTANT * temperature.value) / volume.value,
            ),
            moles,
            temperature,
            materials,
            mass,
            density,
            state: CosmicState::Gas,
        }
    }

    /// Generate a MassDensity from an amount of substance per cubic meter
    pub fn generate_massdensity(
        material: &Molecule,
        molceules_per_cubic_meter: f64,
    ) -> MassDensity {
        MassDensity::new::<kilogram_per_cubic_meter>(
            material.molecular_weight().value * molceules_per_cubic_meter,
        )
    }

    /// Generate a MassDensity from amounts of substances per cubic meter
    pub fn generate_composite_massdensity(
        materials: &Composition,
        molecules_per_cubic_meter: f64,
    ) -> MassDensity {
        let res = materials.0.iter().fold(0.0, |acc, (material, percent)| {
            acc + (material.molecular_weight().value
                * (molecules_per_cubic_meter * (percent / 100.0)))
        });
        MassDensity::new::<kilogram_per_cubic_meter>(res)
    }

    fn construct_from_ideal_properties(
        volume: Volume,
        moles: AmountOfSubstance,
        pressure: Pressure,
        temperature: ThermodynamicTemperature,
        material: Molecule,
    ) -> UniformGas {
        let mass: Mass = material.mass_in_amount(moles);
        let density: MassDensity = density::from_molar_mass_pressure_temperature(
            material.molar_mass(),
            pressure,
            temperature,
        );
        UniformGas {
            volume,
            pressure,
            moles,
            temperature,
            materials: Composition(vec![(material, 100.0)]),
            mass,
            density,
            state: CosmicState::Gas,
        }
    }

    pub fn from_vacuum_properties(
        volume: Volume,
        particles_per_cubic_meter: f64,
        temperature: ThermodynamicTemperature,
        material: Molecule,
    ) -> UniformGas {
        let density: MassDensity =
            UniformGas::generate_massdensity(&material, particles_per_cubic_meter);
        let mass = mass::from_volume_and_density(volume, density);
        let moles: AmountOfSubstance =
            AmountOfSubstance::new::<mole>(mass.value / material.molar_mass().value);
        UniformGas {
            volume,
            pressure: Pressure::new::<pascal>(
                (moles.value * GAS_CONSTANT * temperature.value) / volume.value,
            ),
            moles,
            temperature,
            materials: Composition(vec![(material, 100.0)]),
            mass,
            density,
            state: CosmicState::Gas,
        }
    }

    /// Generate a gas from the properties of an ideal gas.
    /// Provide the volume, moles, temperature, and materials.
    pub fn from_volume_moles_temperature(
        volume: Volume,
        moles: AmountOfSubstance,
        temperature: ThermodynamicTemperature,
        material: Molecule,
    ) -> UniformGas {
        let pressure: Pressure = Pressure::new::<pascal>(
            (moles.value * GAS_CONSTANT * temperature.value) / volume.value,
        );
        UniformGas::construct_from_ideal_properties(volume, moles, pressure, temperature, material)
    }

    /// Generate a gas from the properties of an ideal gas.
    /// Provide the pressure, volume, temperature, and materials.
    pub fn from_pressure_volume_temperature(
        pressure: Pressure,
        volume: Volume,
        temperature: ThermodynamicTemperature,
        material: Molecule,
    ) -> UniformGas {
        let moles = AmountOfSubstance::new::<mole>(pressure.value * volume.value)
            / (GAS_CONSTANT * temperature.value);
        UniformGas::construct_from_ideal_properties(volume, moles, pressure, temperature, material)
    }

    /// Generate a gas from the properties of an ideal gas.
    /// Provide the pressure, moles, temperature, and materials.
    pub fn from_pressure_moles_temperature(
        pressure: Pressure,
        moles: AmountOfSubstance,
        temperature: ThermodynamicTemperature,
        material: Molecule,
    ) -> UniformGas {
        let volume: Volume = Volume::new::<cubic_meter>(
            (moles.value * GAS_CONSTANT * temperature.value) / pressure.value,
        );
        UniformGas::construct_from_ideal_properties(volume, moles, pressure, temperature, material)
    }

    /// Generate a gas from the properties of an ideal gas.
    /// Provide the pressure, volume, moles, and materials.
    pub fn from_pressure_volume_moles(
        pressure: Pressure,
        volume: Volume,
        moles: AmountOfSubstance,
        material: Molecule,
    ) -> UniformGas {
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>(
            (pressure.value * volume.value) / (moles.value * GAS_CONSTANT),
        );
        UniformGas::construct_from_ideal_properties(volume, moles, pressure, temperature, material)
    }

    /// Calculate the jeans mass of this gas entity
    fn jeans_mass(&self) -> Mass {
        let average_particle_mass: Mass = self
            .materials
            .0
            .iter()
            .fold(Mass::new::<kilogram>(0.0), |acc, (material, ratio)| {
                material.molecular_weight() * (ratio / 100.0)
            });
        mass::jeans_mass(self.temperature, average_particle_mass, self.density)
    }

    /// Calculate whether this gas is below the mass threshold for collapsing under it's own gravity.
    /// If the gas is not stable, it will collapse inward at a rate determined by [TODO: rate_of_collapse function]
    pub fn stable(&self) -> bool {
        self.mass < self.jeans_mass()
    }
}

impl Gas for UniformGas {}

pub struct UniformGasAsynchronousOptions {
    temperature_offset: Option<Interpolationf64AsyncOptions>,
}

impl Interpolatable for UniformGas {
    type AO = UniformGasAsynchronousOptions;

    fn interpolate(
        &self,
        target: &Self,
        transition: f64,
        ease: Option<EasingFunction>,
        asynchronous_options: Option<Self::AO>,
    ) -> Self {
        let temperature_interpolation_offset: Option<Interpolationf64AsyncOptions> = {
            match asynchronous_options {
                Some(opts) => opts.temperature_offset,
                None => None,
            }
        };
        let temperature =
            ThermodynamicTemperature::new::<kelvin>(self.temperature.value.interpolate(
                &target.temperature.value,
                transition,
                ease,
                temperature_interpolation_offset,
            ));
        let volume = Volume::new::<cubic_meter>(self.volume.value.interpolate(
            &target.volume.value,
            transition,
            ease,
            None,
        ));
        let pressure = Pressure::new::<pascal>(self.pressure.value.interpolate(
            &target.pressure.value,
            transition,
            ease,
            None,
        ));
        let moles = AmountOfSubstance::new::<mole>(self.moles.value.interpolate(
            &target.moles.value,
            transition,
            ease,
            None,
        ));
        let mass = Mass::new::<kilogram>(self.mass.value.interpolate(
            &target.mass.value,
            transition,
            ease,
            None,
        ));
        let density = MassDensity::new::<kilogram_per_cubic_meter>(self.density.value.interpolate(
            &target.density.value,
            transition,
            ease,
            None,
        ));

        let materials = self
            .materials
            .interpolate(&target.materials, transition, ease, None);

        UniformGas {
            volume,
            pressure,
            moles,
            temperature,
            materials,
            mass,
            density,
            state: self.state.clone(),
        }
    }
}
