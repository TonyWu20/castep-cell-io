use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::FromCellValue;
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Specifies the units in which volume will be reported.
///
/// Keyword type: String
///
/// Default: VolumeUnit::Ang3
///
/// Example:
/// VOLUME_UNIT : nm**3
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "VOLUME_UNIT")]
pub enum VolumeUnit {
    /// Bohr^3
    #[serde(alias = "BOHR**3", alias = "bohr**3")]
    Bohr3,
    /// Meter^3
    #[serde(alias = "M**3", alias = "m**3")]
    Meter3,
    /// Centimeter^3
    #[serde(alias = "CM**3", alias = "cm**3")]
    Centimeter3,
    /// Nanometer^3
    #[serde(alias = "NM**3", alias = "nm**3")]
    Nanometer3,
    /// Ångstrom^3
    #[serde(alias = "ANG**3", alias = "ang**3")]
    #[default]
    Ang3,
}

impl FromCellValue for VolumeUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "bohr**3" => Ok(Self::Bohr3),
            "m**3" => Ok(Self::Meter3),
            "cm**3" => Ok(Self::Centimeter3),
            "nm**3" => Ok(Self::Nanometer3),
            "ang**3" => Ok(Self::Ang3),
            other => Err(Error::Message(format!(
                "unknown VolumeUnit: {other}"
            ))),
        }
    }
}

impl ToCell for VolumeUnit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("VOLUME_UNIT", self.to_cell_value())
    }
}

impl ToCellValue for VolumeUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                VolumeUnit::Bohr3 => "bohr**3",
                VolumeUnit::Meter3 => "m**3",
                VolumeUnit::Centimeter3 => "cm**3",
                VolumeUnit::Nanometer3 => "nm**3",
                VolumeUnit::Ang3 => "ang**3",
            }
            .to_string(),
        )
    }
}


