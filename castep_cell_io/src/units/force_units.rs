// File: force_unit.rs (or part of your main module structure)

use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::FromCellValue;
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Specifies the units in which force will be reported.
///
/// Keyword type: String
///
/// Default: ev/ang
///
/// Example:
/// FORCE_UNIT : n
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "FORCE_UNIT")] // Ensures correct key name during serde
pub enum ForceUnit {
    /// Hartree per Bohr
    #[serde(alias = "HARTREE/BOHR", alias = "hartree/bohr")]
    HartreePerBohr,
    /// Electron volts per Ångström
    #[serde(alias = "EV/ANG", alias = "ev/ang")]
    #[default]
    EvPerAng,
    /// Newton
    #[serde(alias = "N", alias = "n")]
    Newton,
}

// Implement ToCell for ForceUnit to enable serialization via your custom backend
// Note: ForceUnit itself is an enum, not a struct holding a value+unit like SymmetryTol.
// It represents the unit keyword value directly.
// So, ToCell would be used if FORCE_UNIT were a top-level item to serialize on its own,
// though typically it's serialized as part of a larger structure (like SymmetryTol.to_cell_value()).
impl FromCellValue for ForceUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "hartree/bohr" => Ok(Self::HartreePerBohr),
            "ev/ang" => Ok(Self::EvPerAng),
            "n" => Ok(Self::Newton),
            other => Err(Error::Message(format!(
                "unknown ForceUnit: {other}"
            ))),
        }
    }
}

impl ToCell for ForceUnit {
    fn to_cell(&self) -> Cell<'_> {
        // Create a KeyValue Cell with the name "FORCE_UNIT" and the unit string as the value.
        Cell::KeyValue("FORCE_UNIT", self.to_cell_value())
    }
}

// Implement ToCellValue for ForceUnit.
// This is the primary way it will be serialized when used as a value (e.g., in SymmetryTol).
impl ToCellValue for ForceUnit {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                ForceUnit::HartreePerBohr => "hartree/bohr",
                ForceUnit::EvPerAng => "ev/ang",
                ForceUnit::Newton => "n",
            }
            .to_string(), // Convert &str to String for CellValue::String
        )
    }
}


