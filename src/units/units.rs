pub mod length {
    unit! {
        system: uom::si;
        quantity: uom::si::length;

        @earth_radius: 6371.0E+03; "eϴ", "earth_radius", "earth_radii";
        @solar_radius: 6.95700E+08; "sϴ", "solar_radius", "solar_radii";
    }
}

pub mod mass {
    unit! {
        system: uom::si;
        quantity: uom::si::mass;

        @earth_mass: 5.976E+24; "M∅", "earth_mass", "earth_masses";
        @solar_mass: 1.989E+30; "S∅", "solar_mass", "solar_masses";
        // @hydrogen_mass: 1.6735575E-27; "H", "hydrogen_mass", "hydrogen_masses";

        @dalton: 1.6605300000013E-27; "Da", "dalton", "daltons";
    }
}

// pub mod mass_density {
//     unit! {
//         system: uom::si;
//         quantity: uom::si::mass_density;
//
//         @hydrogen_atom_per_cubic_meter: 1.67E-24; "H^M-3", "hydrogen_per_cubic_meter", "hydrogen_atoms_per_cubic_meter";
//         @hydrogen_atom_per_cubic_centimeter: 1.67E-27; "H^CM-3", "hydrogen_per_cubic_centimeter", "hydrogen_atoms_per_cubic_centimeter";
//     }
// }

pub mod power {
    unit! {
        system: uom::si;
        quantity: uom::si::power;

        @solar_luminosity: 3.84392751142533078162625368E+26; "L☉", "solar_luminosity", "solar_luminosities";
    }
}

pub mod time {
    unit! {
        system: uom::si;
        quantity: uom::si::time;

        @million_year: 31536000000000.0; "Ma", "million_year", "million_years";
        @thousand_year: 31536000000.0; "Ta", "thousand_year", "thousand_years";
        @century: 3155760000.0; "cy", "century", "centuries";
        @billion_year: 31536000000000000.0; "Ga", "billion_year", "billion_years";
    }
}

pub mod acceleration {
    unit! {
        system: uom::si;
        quantity: uom::si::acceleration;

        @earth_gravity: 9.8; "N", "earth_gravity", "earth_gravities";
    }
}

pub mod pressure {
    unit! {
        system: uom::si;
        quantity: uom::si::pressure;

        @atmosphere: 101325.0; "atm", "atmosphere", "atmospheres";
    }
}

pub mod volume {
    unit! {
        system: uom::si;
        quantity: uom::si::volume;

        @cubic_lightyear: 9.46073047258E15; "ly^3", "cubic_lightyear", "cubic_lightyears";
    }
}
