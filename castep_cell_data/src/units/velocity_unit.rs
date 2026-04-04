use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::FromCellValue;
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Specifies the units in which velocity will be reported.
///
/// Keyword type: String
///
/// Default: VelocityUnit::AngPerPs
///
/// Example:
/// VELOCITY_UNIT : bohr/fs
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "VELOCITY_UNIT")]
pub enum VelocityUnit {
    /// Atomic unit of velocity
    #[serde(alias = "AUV", alias = "auv")]
    AtomicUnitOfVelocity,
    /// Å/ps
    #[serde(alias = "ANG/PS", alias = "ang/ps")]
    #[default]
    AngPerPs,
    /// Å/fs
    #[serde(alias = "ANG/FS", alias = "ang/fs")]
    AngPerFs,
    /// Bohr per picosecond
    #[serde(alias = "BOHR/PS", alias = "bohr/ps")]
    BohrPerPs,
    /// Bohr per femtosecond
    #[serde(alias = "BOHR/FS", alias = "bohr/fs")]
    BohrPerFs,
    /// Meters per second
    #[serde(alias = "M/S", alias = "m/s")]
    MetersPerSecond,
}

impl FromCellValue for VelocityUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "auv" => Ok(Self::AtomicUnitOfVelocity),
            "ang/ps" => Ok(Self::AngPerPs),
            "ang/fs" => Ok(Self::AngPerFs),
            "bohr/ps" => Ok(Self::BohrPerPs),
            "bohr/fs" => Ok(Self::BohrPerFs),
            "m/s" => Ok(Self::MetersPerSecond),
            other => Err(Error::Message(format!(
                "unknown VelocityUnit: {other}"
            ))),
        }
    }
}

impl ToCell for VelocityUnit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("VELOCITY_UNIT", self.to_cell_value())
    }
}

impl ToCellValue for VelocityUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                VelocityUnit::AtomicUnitOfVelocity => "auv",
                VelocityUnit::AngPerPs => "ang/ps",
                VelocityUnit::AngPerFs => "ang/fs",
                VelocityUnit::BohrPerPs => "bohr/ps",
                VelocityUnit::BohrPerFs => "bohr/fs",
                VelocityUnit::MetersPerSecond => "m/s",
            }
            .to_string(),
        )
    }
}


