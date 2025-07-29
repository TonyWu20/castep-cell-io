use castep_cell_serde::{CellValue, ToCellValue};
use serde::{Deserialize, Serialize};

mod hubbard_u;
mod quantization_axis;
mod species_lcao_states;
mod species_mass;
mod species_pot;
// future
// mod species_q;

pub use hubbard_u::{AtomHubbardU, HubbardU, HubbardUUnit, OrbitalU};
pub use quantization_axis::QuantizationAxis;
pub use species_lcao_states::{SpeciesLcaoState, SpeciesLcaoStates};
pub use species_mass::{SpeciesMass, SpeciesMassEntry};
pub use species_pot::{SpeciesPot, SpeciesPotEntry};

/// Represents the species identifier for an atom in a `POSITIONS_*` block.
/// Can be either a chemical symbol (e.g., "Fe") or an atomic number (e.g., 26).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Species {
    /// Chemical symbol (e.g., "Fe", "Si", "O").
    Symbol(String), // Using String for flexibility, though CASTEP limits to 3 chars
    /// Atomic number (e.g., 26 for Iron).
    AtomicNumber(u32),
}

impl Species {
    pub fn as_symbol(&self) -> Option<&String> {
        if let Self::Symbol(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_atomic_number(&self) -> Option<&u32> {
        if let Self::AtomicNumber(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl ToCellValue for Species {
    fn to_cell_value(&self) -> CellValue {
        match self {
            Species::Symbol(s) => CellValue::String(s.clone()),
            Species::AtomicNumber(u) => CellValue::UInt(*u),
        }
    }
}
