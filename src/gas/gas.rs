use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::amount_of_substance::mole;
use uom::si::f64::{Mass, Volume, MassDensity, AmountOfSubstance, Pressure, ThermodynamicTemperature};
use uom::si::mass::kilogram;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::pressure::pascal;
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::volume::cubic_meter;
use crate::chemistry::molecules::molecules::Molecule;
use crate::formulae::constants::GAS_CONSTANT;
use crate::formulae::formulae::{density, mass};
use crate::solar_mass;
use crate::transition::transition::Transition;

pub struct Gas {
    pub volume: Volume,
    pub pressure: Pressure,
    pub moles: AmountOfSubstance,
    pub temperature: ThermodynamicTemperature,

    pub materials: Vec<(Molecule, f64)>,

    pub mass: Mass,
    pub density: MassDensity
}

impl Gas {
    /// Generate a MassDensity from an amount of substance per cubic meter
    pub fn generate_massdensity(material: &Molecule, molceules_per_cubic_meter: f64) -> MassDensity {
        MassDensity::new::<kilogram_per_cubic_meter>(material.molecular_weight().value * molceules_per_cubic_meter)
    }

    /// Generate a MassDensity from amounts of substances per cubic meter
    pub fn generate_composite_massdensity(materials: &Vec<(Molecule, f64)>, molecules_per_cubic_meter: f64) -> MassDensity {
        let res = materials.iter().fold(0.0, |acc, (material, percent), | {
            acc + (material.molecular_weight().value * (molecules_per_cubic_meter * (percent / 100.0)))
        });
        MassDensity::new::<kilogram_per_cubic_meter>(res)
    }

    fn construct_from_ideal_properties(volume: Volume, moles: AmountOfSubstance, pressure: Pressure, temperature: ThermodynamicTemperature, material: Molecule) -> Gas {
        let mass: Mass = material.mass_in_amount(moles);
        let density: MassDensity = density::from_molar_mass_pressure_temperature(material.molar_mass(), pressure, temperature);
        Gas {
            volume,
            pressure,
            moles,
            temperature,
            materials: vec!((material, 100.0)),
            mass,
            density
        }
    }

    pub fn from_vacuum_properties(volume: Volume, particles_per_cubic_meter: f64, temperature: ThermodynamicTemperature, material: Molecule) -> Gas {
        let density: MassDensity = Gas::generate_massdensity(&material, particles_per_cubic_meter);
        let mass = mass::from_volume_and_density(volume, density);
        let moles: AmountOfSubstance = AmountOfSubstance::new::<mole>(mass.value / material.molar_mass().value);
        Gas {
            volume,
            pressure: Pressure::new::<pascal>((moles.value * GAS_CONSTANT * temperature.value) / volume.value),
            moles,
            temperature,
            materials: vec!((material, 100.0)),
            mass,
            density
        }
    }

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
    pub fn composite_from_vacuum_properties(volume: Volume, particles_per_cubic_meter: f64, temperature: ThermodynamicTemperature, materials: Vec<(Molecule, f64)>) -> Gas {
        let density: MassDensity = Gas::generate_composite_massdensity(&materials, particles_per_cubic_meter);
        let mass = mass::from_volume_and_density(volume, density);
        let moles: AmountOfSubstance = materials.iter().fold(AmountOfSubstance::new::<mole>(0.0), |acc, (material, ratio)| {
            acc + AmountOfSubstance::new::<mole>((mass.value / material.molar_mass().value) * (ratio / 100.0))
        });
        Gas {
            volume,
            pressure: Pressure::new::<pascal>((moles.value * GAS_CONSTANT * temperature.value) / volume.value),
            moles,
            temperature,
            materials,
            mass,
            density
        }
    }

    /// Generate a gas from the properties of an ideal gas.
    /// Provide the volume, moles, temperature, and materials.
    pub fn from_volume_moles_temperature(volume: Volume, moles: AmountOfSubstance, temperature: ThermodynamicTemperature, material: Molecule) -> Gas {
        let pressure: Pressure = Pressure::new::<pascal>((moles.value * GAS_CONSTANT * temperature.value) / volume.value);
        Gas::construct_from_ideal_properties(volume, moles, pressure, temperature, material)
    }

    /// Generate a gas from the properties of an ideal gas.
    /// Provide the pressure, volume, temperature, and materials.
    pub fn from_pressure_volume_temperature(pressure: Pressure, volume: Volume, temperature: ThermodynamicTemperature, material: Molecule) -> Gas {
        let moles = AmountOfSubstance::new::<mole>(pressure.value * volume.value) / (GAS_CONSTANT * temperature.value);
        Gas::construct_from_ideal_properties(volume, moles, pressure, temperature, material)
    }

    /// Generate a gas from the properties of an ideal gas.
    /// Provide the pressure, moles, temperature, and materials.
    pub fn from_pressure_moles_temperature(pressure: Pressure, moles: AmountOfSubstance, temperature: ThermodynamicTemperature, material: Molecule) -> Gas {
        let volume: Volume = Volume::new::<cubic_meter>((moles.value * GAS_CONSTANT * temperature.value) / pressure.value);
        Gas::construct_from_ideal_properties(volume, moles, pressure, temperature, material)
    }

    /// Generate a gas from the properties of an ideal gas.
    /// Provide the pressure, volume, moles, and materials.
    pub fn from_pressure_volume_moles(pressure: Pressure, volume: Volume, moles: AmountOfSubstance, material: Molecule) -> Gas {
        let temperature: ThermodynamicTemperature = ThermodynamicTemperature::new::<kelvin>((pressure.value * volume.value) / (moles.value * GAS_CONSTANT));
        Gas::construct_from_ideal_properties(volume, moles, pressure, temperature, material)

    }

    /// Calculate the jeans mass of this gas entity
    fn jeans_mass(&self) -> Mass {
        let average_particle_mass: Mass = self.materials.iter().fold(Mass::new::<kilogram>(0.0), |acc, (material, ratio)| {
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

impl Transition for Gas {
    type OutputSelf = Gas;
    const S: usize = 6;

    fn get_interpolatable_values_array(&self) -> [f64; Self::S] {
        [self.temperature.value, self.density.value, self.mass.value, self.volume.value, self.moles.value, self.pressure.value]
    }

    fn apply_interpolatable_values_array(&self, values: [f64; Self::S]) -> Self {
        Gas {
            temperature: ThermodynamicTemperature::new::<kelvin>(*values.get(0).unwrap()),
            density: MassDensity::new::<kilogram_per_cubic_meter>(*values.get(1).unwrap()),
            mass: Mass::new::<kilogram>(*values.get(2).unwrap()),
            volume: Volume::new::<cubic_meter>(*values.get(3).unwrap()),
            moles: AmountOfSubstance::new::<mole>(*values.get(4).unwrap()),
            pressure: Pressure::new::<pascal>(*values.get(5).unwrap()),
            materials: self.materials.clone()
        }
    }
}

