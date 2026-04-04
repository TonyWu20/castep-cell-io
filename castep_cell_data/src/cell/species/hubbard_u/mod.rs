mod atom_hubbard_u_aux;
mod hubbard_u_aux;

use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, parse::FromCellValue, CResult, Error, query::value_as_str};
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
    fn to_cell_value(&self) -> CellValue {
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
    fn to_cell_value(&self) -> CellValue {
        // Format as "l: U" e.g., "d: 2.3"
        CellValue::String(format!("{}: {}", self.orbital_char(), self.u_value()))
    }
}

/// Represents the specification for Hubbard U values for a specific atom/ion.
#[derive(Debug, Clone, PartialEq)]
pub struct AtomHubbardU {
    /// The species.
    pub species: Species,
    /// The optional ion number within the species (1-based index).
    /// If None, the U values apply to all ions of this species.
    pub ion_number: Option<u32>,
    /// The list of orbitals and their U values.
    pub orbitals: Vec<OrbitalU>,
}

impl ToCellValue for AtomHubbardU {
    fn to_cell_value(&self) -> CellValue {
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
#[derive(Debug, Clone, PartialEq)]
pub struct HubbardU {
    /// The unit for U values. If None, the default (eV) is used.
    pub unit: Option<HubbardUUnit>,
    /// The list of atom-specific Hubbard U specifications.
    pub atom_u_values: Vec<AtomHubbardU>,
}

impl ToCell for HubbardU {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell {
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


