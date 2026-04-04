use castep_cell_fmt::{CellValue, ToCellValue};
use castep_cell_fmt::parse::FromCellValue;
use castep_cell_fmt::{CResult, Error};
use serde::{Deserialize, Serialize};

mod hubbard_u;
mod quantization_axis;
mod sedc_custom_params;
mod species_lcao_states;
mod species_mass;
mod species_pot;
mod species_q;

pub use hubbard_u::{AtomHubbardU, HubbardU, HubbardUUnit, OrbitalU};
pub use quantization_axis::QuantizationAxis;
pub use sedc_custom_params::{SedcCustomParams, SedcCustomParamsEntry, SedcParameter};
pub use species_lcao_states::{SpeciesLcaoState, SpeciesLcaoStates};
pub use species_mass::{SpeciesMass, SpeciesMassEntry};
pub use species_pot::{SpeciesPot, SpeciesPotEntry};
pub use species_q::{SpeciesQ, SpeciesQEntry};

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

impl FromCellValue for Species {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Str(s) => {
                if let Ok(n) = s.parse::<u32>() {
                    Ok(Species::AtomicNumber(n))
                } else {
                    Ok(Species::Symbol(s.to_string()))
                }
            }
            CellValue::UInt(n) => Ok(Species::AtomicNumber(*n)),
            CellValue::String(s) => Ok(Species::Symbol(s.clone())),
            other => Err(Error::UnexpectedType(
                "Species".into(),
                format!("{other:?}"),
            )),
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

impl std::fmt::Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Species::Symbol(s) => write!(f, "{s}"),
            Species::AtomicNumber(n) => write!(f, "{n}"),
        }
    }
}
