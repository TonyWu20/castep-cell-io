use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;

/// Determines the thermostat used for a molecular dynamics calculation (NVT ensemble).
///
/// Keyword type: String
///
/// Default: MdThermostat::NoseHoover
///
/// Example:
/// MD_THERMOSTAT : Langevin
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MdThermostat {
    /// Nosé-Hoover thermostat
    NoseHoover,
    /// Langevin thermostat
    Langevin,
}

impl Default for MdThermostat {
    fn default() -> Self {
        Self::NoseHoover // Default is Nosé-Hoover
    }
}

impl FromCellValue for MdThermostat {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "nosé-hoover" | "nose-hoover" => Ok(Self::NoseHoover),
            "langevin" => Ok(Self::Langevin),
            other => Err(Error::Message(format!("unknown MdThermostat: {other}"))),
        }
    }
}

impl FromKeyValue for MdThermostat {
    const KEY_NAME: &'static str = "MD_THERMOSTAT";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdThermostat {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_THERMOSTAT", self.to_cell_value())
    }
}

impl ToCellValue for MdThermostat {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MdThermostat::NoseHoover => "Nosé-Hoover",
                MdThermostat::Langevin => "Langevin",
            }
            .to_string(),
        )
    }
}


