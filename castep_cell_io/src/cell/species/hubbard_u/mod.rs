mod atom_hubbard_u_aux;
mod hubbard_u_aux;

use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, parse::FromCellValue, CResult, Error, query::value_as_str};
use super::Species;

/// Represents the unit for Hubbard U values.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum HubbardUUnit {
    /// Electron volts
    #[default]
    ElectronVolt,
    /// Hartree
    Hartree,
}

impl FromCellValue for HubbardUUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "ev" => Ok(Self::ElectronVolt),
            "ha" => Ok(Self::Hartree),
            other => Err(Error::Message(format!("unknown HubbardUUnit: {other}"))),
        }
    }
}

impl ToCellValue for HubbardUUnit {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                HubbardUUnit::ElectronVolt => "eV",
                HubbardUUnit::Hartree => "Ha",
            }
            .to_string(),
        )
    }
}

/// Represents an orbital type and its associated Hubbard U value.
#[derive(Debug, Clone, PartialEq)]
pub enum OrbitalU {
    S(f64),
    P(f64),
    D(f64),
    F(f64),
}

impl OrbitalU {
    /// Gets the orbital type as a character.
    pub fn orbital_char(&self) -> char {
        match self {
            OrbitalU::S(_) => 's',
            OrbitalU::P(_) => 'p',
            OrbitalU::D(_) => 'd',
            OrbitalU::F(_) => 'f',
        }
    }

    /// Gets the U value.
    pub fn u_value(&self) -> f64 {
        match self {
            OrbitalU::S(v) => *v,
            OrbitalU::P(v) => *v,
            OrbitalU::D(v) => *v,
            OrbitalU::F(v) => *v,
        }
    }

    fn from_str_f64(orbital_str: &str, val: f64) -> Option<Self> {
        match orbital_str.trim_end_matches([' ', ':', '=']) {
            "s" => Some(Self::S(val)),
            "p" => Some(Self::P(val)),
            "d" => Some(Self::D(val)),
            "f" => Some(Self::F(val)),
            _ => None,
        }
    }
}

impl ToCellValue for OrbitalU {
    fn to_cell_value(&self) -> CellValue<'_> {
        // Format as "l: U" e.g., "d: 2.3"
        CellValue::String(format!("{}: {}", self.orbital_char(), self.u_value()))
    }
}

/// Represents the specification for Hubbard U values for a specific atom/ion.
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct AtomHubbardU {
    /// The species.
    pub species: Species,
    /// The optional ion number within the species (1-based index).
    /// If None, the U values apply to all ions of this species.
    pub ion_number: Option<u32>,
    /// The list of orbitals and their U values.
    #[builder(default)]
    pub orbitals: Vec<OrbitalU>,
}

impl ToCellValue for AtomHubbardU {
    fn to_cell_value(&self) -> CellValue<'_> {
        let line_parts = [
            self.species.to_cell_value(),
            self.ion_number
                .map(CellValue::UInt)
                .unwrap_or(CellValue::Null),
        ]
        .to_vec()
        .into_iter()
        .chain(self.orbitals.iter().map(|orb| orb.to_cell_value()))
        .collect();
        CellValue::Array(line_parts)
    }
}

/// Represents the HUBBARD_U block.
///
/// Defines the Hubbard U values to use for specific orbitals.
/// Format:
/// %BLOCK HUBBARD_U
/// [UNITS]
/// atom1 orbital1 orbital2 ....
/// atom2 orbital1 orbital2 ....
/// ...
/// %ENDBLOCK HUBBARD_U
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct HubbardU {
    /// The unit for U values. If None, the default (eV) is used.
    pub unit: Option<HubbardUUnit>,
    /// The list of atom-specific Hubbard U specifications.
    #[builder(default)]
    pub atom_u_values: Vec<AtomHubbardU>,
}

impl ToCell for HubbardU {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell<'_> {
        let mut block_content = Vec::new();

        // Add the optional unit line first
        if let Some(ref u) = self.unit {
            block_content.push(CellValue::Array(vec![u.to_cell_value()]));
        }

        // Add the atom-specific lines
        block_content.extend(
            self.atom_u_values
                .iter()
                .map(|atom_u| atom_u.to_cell_value()),
        );

        Cell::Block("HUBBARD_U", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom_hubbard_u_builder_basic() {
        let species = Species::Symbol("Fe".to_string());
        let orbital = OrbitalU::D(5.0);

        let atom_u = AtomHubbardU::builder()
            .species(species.clone())
            .ion_number(1)
            .orbitals(vec![orbital.clone()])
            .build();

        assert_eq!(atom_u.species, species);
        assert_eq!(atom_u.ion_number, Some(1));
        assert_eq!(atom_u.orbitals.len(), 1);
        assert_eq!(atom_u.orbitals[0], orbital);
    }

    #[test]
    fn test_atom_hubbard_u_builder_no_ion_number() {
        let species = Species::Symbol("O".to_string());
        let orbital = OrbitalU::P(3.0);

        let atom_u = AtomHubbardU::builder()
            .species(species.clone())
            .orbitals(vec![orbital.clone()])
            .build();

        assert_eq!(atom_u.species, species);
        assert_eq!(atom_u.ion_number, None);
        assert_eq!(atom_u.orbitals.len(), 1);
    }

    #[test]
    fn test_atom_hubbard_u_builder_push_orbitals() {
        let species = Species::Symbol("Fe".to_string());
        let orbital_d = OrbitalU::D(5.0);
        let orbital_p = OrbitalU::P(2.0);

        let mut atom_u = AtomHubbardU::builder()
            .species(species.clone())
            .build();
        atom_u.orbitals.push(orbital_d.clone());
        atom_u.orbitals.push(orbital_p.clone());

        assert_eq!(atom_u.orbitals.len(), 2);
        assert_eq!(atom_u.orbitals[0], orbital_d);
        assert_eq!(atom_u.orbitals[1], orbital_p);
    }

    #[test]
    fn test_atom_hubbard_u_builder_multiple_push_calls() {
        let species = Species::Symbol("Ni".to_string());
        let orbitals = vec![OrbitalU::D(6.0), OrbitalU::S(1.0), OrbitalU::P(2.5)];

        let mut atom_u = AtomHubbardU::builder().species(species.clone()).build();
        for orbital in &orbitals {
            atom_u.orbitals.push(orbital.clone());
        }

        assert_eq!(atom_u.orbitals.len(), 3);
        assert_eq!(atom_u.orbitals, orbitals);
    }

    #[test]
    fn test_hubbard_u_builder_basic() {
        let species = Species::Symbol("Fe".to_string());
        let atom_u = AtomHubbardU::builder()
            .species(species)
            .orbitals(vec![OrbitalU::D(5.0)])
            .build();

        let hubbard_u = HubbardU::builder()
            .unit(HubbardUUnit::ElectronVolt)
            .atom_u_values(vec![atom_u.clone()])
            .build();

        assert_eq!(hubbard_u.unit, Some(HubbardUUnit::ElectronVolt));
        assert_eq!(hubbard_u.atom_u_values.len(), 1);
        assert_eq!(hubbard_u.atom_u_values[0], atom_u);
    }

    #[test]
    fn test_hubbard_u_builder_no_unit() {
        let species = Species::Symbol("O".to_string());
        let atom_u = AtomHubbardU::builder()
            .species(species)
            .orbitals(vec![OrbitalU::P(3.0)])
            .build();

        let hubbard_u = HubbardU::builder()
            .atom_u_values(vec![atom_u.clone()])
            .build();

        assert_eq!(hubbard_u.unit, None);
        assert_eq!(hubbard_u.atom_u_values.len(), 1);
    }

    #[test]
    fn test_hubbard_u_builder_push_atom_u_values() {
        let species_fe = Species::Symbol("Fe".to_string());
        let species_o = Species::Symbol("O".to_string());

        let atom_u_fe = AtomHubbardU::builder()
            .species(species_fe)
            .orbitals(vec![OrbitalU::D(5.0)])
            .build();

        let atom_u_o = AtomHubbardU::builder()
            .species(species_o)
            .orbitals(vec![OrbitalU::P(3.0)])
            .build();

        let mut hubbard_u = HubbardU::builder()
            .unit(HubbardUUnit::Hartree)
            .build();
        hubbard_u.atom_u_values.push(atom_u_fe.clone());
        hubbard_u.atom_u_values.push(atom_u_o.clone());

        assert_eq!(hubbard_u.atom_u_values.len(), 2);
        assert_eq!(hubbard_u.atom_u_values[0], atom_u_fe);
        assert_eq!(hubbard_u.atom_u_values[1], atom_u_o);
    }

    #[test]
    fn test_hubbard_u_builder_multiple_push_calls() {
        let atom_u_values = vec![
            AtomHubbardU::builder()
                .species(Species::Symbol("Fe".to_string()))
                .orbitals(vec![OrbitalU::D(5.0)])
                .build(),
            AtomHubbardU::builder()
                .species(Species::Symbol("O".to_string()))
                .orbitals(vec![OrbitalU::P(3.0)])
                .build(),
            AtomHubbardU::builder()
                .species(Species::Symbol("Ni".to_string()))
                .orbitals(vec![OrbitalU::D(6.0)])
                .build(),
        ];

        let mut hubbard_u = HubbardU::builder().unit(HubbardUUnit::ElectronVolt).build();
        for atom_u in &atom_u_values {
            hubbard_u.atom_u_values.push(atom_u.clone());
        }

        assert_eq!(hubbard_u.atom_u_values.len(), 3);
        assert_eq!(hubbard_u.atom_u_values, atom_u_values);
    }

    #[test]
    fn test_hubbard_u_builder_empty_atom_u_values() {
        let hubbard_u = HubbardU::builder()
            .unit(HubbardUUnit::ElectronVolt)
            .build();

        assert_eq!(hubbard_u.unit, Some(HubbardUUnit::ElectronVolt));
        assert_eq!(hubbard_u.atom_u_values.len(), 0);
    }

    #[test]
    fn test_atom_hubbard_u_builder_empty_orbitals() {
        let species = Species::Symbol("Fe".to_string());

        let atom_u = AtomHubbardU::builder()
            .species(species.clone())
            .build();

        assert_eq!(atom_u.species, species);
        assert_eq!(atom_u.orbitals.len(), 0);
    }

    #[test]
    fn test_hubbard_u_builder_mixed_initialization() {
        let atom_u_fe = AtomHubbardU::builder()
            .species(Species::Symbol("Fe".to_string()))
            .orbitals(vec![OrbitalU::D(5.0)])
            .build();

        let atom_u_o = AtomHubbardU::builder()
            .species(Species::Symbol("O".to_string()))
            .orbitals(vec![OrbitalU::P(3.0)])
            .build();

        let mut hubbard_u = HubbardU::builder()
            .atom_u_values(vec![atom_u_fe.clone()])
            .build();
        hubbard_u.atom_u_values.push(atom_u_o.clone());

        assert_eq!(hubbard_u.atom_u_values.len(), 2);
        assert_eq!(hubbard_u.atom_u_values[0], atom_u_fe);
        assert_eq!(hubbard_u.atom_u_values[1], atom_u_o);
    }

    #[test]
    fn test_hubbard_u_builder_method_chaining() {
        let atom_u_fe = AtomHubbardU::builder()
            .species(Species::Symbol("Fe".to_string()))
            .orbitals(vec![OrbitalU::D(5.0)])
            .build();

        let atom_u_o = AtomHubbardU::builder()
            .species(Species::Symbol("O".to_string()))
            .orbitals(vec![OrbitalU::P(3.0)])
            .build();

        let hubbard_u = HubbardU::builder()
            .unit(HubbardUUnit::Hartree)
            .atom_u_values(vec![atom_u_fe, atom_u_o])
            .build();

        assert_eq!(hubbard_u.unit, Some(HubbardUUnit::Hartree));
        assert_eq!(hubbard_u.atom_u_values.len(), 2);
    }
}

