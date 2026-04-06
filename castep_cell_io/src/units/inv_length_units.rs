use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::FromCellValue;
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Specifies the units in which inverse length will be reported.
///
/// Keyword type: String
///
/// Default: 1/ang
///
/// Example:
/// INV_LENGTH_UNIT : 1/nm
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "INV_LENGTH_UNIT")] // Ensures correct key name during serde
pub enum InvLengthUnit {
    /// Bohr-1
    #[serde(alias = "1/BOHR", alias = "1/bohr")]
    Bohr,
    /// Meter-1
    #[serde(alias = "1/M", alias = "1/m")]
    Meter,
    /// Nanometer-1
    #[serde(alias = "1/NM", alias = "1/nm")]
    NanoMeter,
    /// Å-1
    #[serde(alias = "1/ANG", alias = "1/ang")]
    #[default]
    Angstrom,
}

// Implement ToCell for InvLengthUnit to enable serialization via your custom backend
impl FromCellValue for InvLengthUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "1/bohr" => Ok(Self::Bohr),
            "1/m" => Ok(Self::Meter),
            "1/nm" => Ok(Self::NanoMeter),
            "1/ang" => Ok(Self::Angstrom),
            other => Err(Error::Message(format!(
                "unknown InvLengthUnit: {other}"
            ))),
        }
    }
}

impl ToCell for InvLengthUnit {
    fn to_cell(&self) -> Cell<'_> {
        // Create a KeyValue Cell with the name "INV_LENGTH_UNIT" and the unit string as the value.
        Cell::KeyValue("INV_LENGTH_UNIT", self.to_cell_value())
    }
}

// Implement ToCellValue for InvLengthUnit.
impl ToCellValue for InvLengthUnit {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                InvLengthUnit::Bohr => "1/bohr",
                InvLengthUnit::Meter => "1/m",
                InvLengthUnit::NanoMeter => "1/nm",
                InvLengthUnit::Angstrom => "1/ang",
            }
            .to_string(), // Convert &str to String for CellValue::String
        )
    }
}


