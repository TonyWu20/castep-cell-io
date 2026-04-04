use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::FromCellValue;
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_string;
use serde::{Deserialize, Serialize};

/// Specifies the units in which force constants will be reported.
///
/// Keyword type: String
///
/// Default: ForceConstantUnit::EvPerAng2
///
/// Example:
/// FORCE_CONSTANT_UNIT : n/m
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "FORCE_CONSTANT_UNIT")]
pub enum ForceConstantUnit {
    /// Hartree per Bohr^2
    #[serde(rename = "hartree/bohr**2", alias = "HARTREE/BOHR**2")]
    HartreePerBohr2,
    /// Electron Volts per Å^2
    #[serde(rename = "ev/ang**2", alias = "EV/ANG**2")]
    #[default]
    EvPerAng2,
    /// Newton per meter
    #[serde(rename = "n/m", alias = "N/M")]
    NewtonPerMeter,
    /// Dynes per centimeter
    #[serde(rename = "dyne/cm", alias = "DYNE/CM")]
    DynesPerCentimeter,
}

impl FromCellValue for ForceConstantUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_string(value)?.to_ascii_lowercase().as_str() {
            "hartree/bohr**2" => Ok(Self::HartreePerBohr2),
            "ev/ang**2" => Ok(Self::EvPerAng2),
            "n/m" => Ok(Self::NewtonPerMeter),
            "dyne/cm" => Ok(Self::DynesPerCentimeter),
            other => Err(Error::Message(format!(
                "unknown ForceConstantUnit: {other}"
            ))),
        }
    }
}

impl ToCell for ForceConstantUnit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("FORCE_CONSTANT_UNIT", self.to_cell_value())
    }
}

impl ToCellValue for ForceConstantUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                ForceConstantUnit::HartreePerBohr2 => "hartree/bohr**2",
                ForceConstantUnit::EvPerAng2 => "ev/ang**2",
                ForceConstantUnit::NewtonPerMeter => "n/m",
                ForceConstantUnit::DynesPerCentimeter => "dyne/cm",
            }
            .to_string(),
        )
    }
}


