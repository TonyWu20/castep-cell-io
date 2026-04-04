use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;

/// Determines the ensemble used for a molecular dynamics calculation.
///
/// Keyword type: String
///
/// Default: MdEnsemble::Nve
///
/// Example:
/// MD_ENSEMBLE : NVT
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MdEnsemble {
    /// Canonical ensemble (constant number of particles, volume, temperature)
    Nvt,
    /// Microcanonical ensemble (constant number of particles, volume, energy)
    Nve,
    /// Isothermal-isobaric ensemble (constant number of particles, pressure, temperature)
    Npt,
    /// Isenthalpic-isobaric ensemble (constant number of particles, pressure, enthalpy)
    Nph,
}

impl Default for MdEnsemble {
    fn default() -> Self {
        Self::Nve // Default is NVE
    }
}

impl FromCellValue for MdEnsemble {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "nvt" => Ok(Self::Nvt),
            "nve" => Ok(Self::Nve),
            "npt" => Ok(Self::Npt),
            "nph" => Ok(Self::Nph),
            other => Err(Error::Message(format!("unknown MdEnsemble: {other}"))),
        }
    }
}

impl FromKeyValue for MdEnsemble {
    const KEY_NAME: &'static str = "MD_ENSEMBLE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdEnsemble {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_ENSEMBLE", self.to_cell_value())
    }
}

impl ToCellValue for MdEnsemble {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MdEnsemble::Nvt => "NVT",
                MdEnsemble::Nve => "NVE",
                MdEnsemble::Npt => "NPT",
                MdEnsemble::Nph => "NPH",
            }
            .to_string(),
        )
    }
}


