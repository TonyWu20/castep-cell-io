use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::FromCellValue;
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Represents the unit of mass.
///
/// Keyword type: String
///
/// Default: amu
///
/// Example: MASS_UNIT : kg
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default, // Derive Default
)]
// #[serde(rename = "MASS_UNIT")] // Ensures correct key name during serde if used directly
pub enum MassUnit {
    /// Electron mass
    #[serde(alias = "ME", alias = "me")]
    ElectronMass,
    /// Atomic mass unit
    #[default] // Specifies the default variant
    #[serde(alias = "AMU", alias = "amu")]
    AtomicMassUnit,
    /// Kilogram
    #[serde(alias = "KG", alias = "kg")]
    Kilogram,
    /// Gram
    #[serde(alias = "G", alias = "g")]
    Gram,
}

impl FromCellValue for MassUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "me" => Ok(Self::ElectronMass),
            "amu" => Ok(Self::AtomicMassUnit),
            "kg" => Ok(Self::Kilogram),
            "g" => Ok(Self::Gram),
            other => Err(Error::Message(format!(
                "unknown MassUnit: {other}"
            ))),
        }
    }
}

impl ToCellValue for MassUnit {
    /// Converts the enum variant into the corresponding `CellValue::String`.
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                MassUnit::ElectronMass => "me",
                MassUnit::AtomicMassUnit => "amu",
                MassUnit::Kilogram => "kg",
                MassUnit::Gram => "g",
            }
            .to_string(), // Convert &str to String
        )
    }
}

impl ToCell for MassUnit {
    fn to_cell(&self) -> castep_cell_fmt::Cell<'_> {
        Cell::KeyValue("MASS_UNIT", self.to_cell_value())
    }
}


