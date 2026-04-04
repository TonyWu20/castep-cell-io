use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;

/// Determines the barostat method used for molecular dynamics with variable cell volume.
///
/// Keyword type: String
///
/// Default: MdBarostat::AndersenHoover
///
/// Example:
/// MD_BAROSTAT : Andersen-Hoover
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MdBarostat {
    /// Andersen-Hoover barostat
    AndersenHoover,
    /// Parrinello-Rahman barostat
    ParrinelloRahman,
}

impl Default for MdBarostat {
    fn default() -> Self {
        Self::AndersenHoover // Default is Andersen-Hoover
    }
}

impl FromCellValue for MdBarostat {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "andersen-hoover" => Ok(Self::AndersenHoover),
            "parrinello-rahman" => Ok(Self::ParrinelloRahman),
            other => Err(Error::Message(format!("unknown MdBarostat: {other}"))),
        }
    }
}

impl FromKeyValue for MdBarostat {
    const KEY_NAME: &'static str = "MD_BAROSTAT";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdBarostat {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_BAROSTAT", self.to_cell_value())
    }
}

impl ToCellValue for MdBarostat {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MdBarostat::AndersenHoover => "Andersen-Hoover",
                MdBarostat::ParrinelloRahman => "Parrinello-Rahman",
            }
            .to_string(),
        )
    }
}


