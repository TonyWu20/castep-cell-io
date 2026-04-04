use castep_cell_io::{Cell, CellValue, Error, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult};
use castep_cell_io::query::value_as_i32;
use serde::{Deserialize, Serialize};

/// Determines whether or not to apply a finite basis set correction
/// to energy and stress when cell parameters change.
///
/// Keyword type: Integer
///
/// Default: 2
///
/// Example:
/// FINITE_BASIS_CORR : 1
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "FINITE_BASIS_CORR")]
pub enum FiniteBasisCorr {
    /// No correction.
    #[serde(rename = "0")]
    None,
    /// Manual correction using specified BASIS_DE_DLOGE.
    #[serde(rename = "1")]
    Manual,
    /// Automatic correction using FINITE_BASIS_NPOINTS and FINITE_BASIS_SPACING.
    #[serde(rename = "2")]
    Automatic,
}

impl Default for FiniteBasisCorr {
    fn default() -> Self {
        Self::Automatic
    }
}

impl FromCellValue for FiniteBasisCorr {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_i32(value)? {
            0 => Ok(Self::None),
            1 => Ok(Self::Manual),
            2 => Ok(Self::Automatic),
            n => Err(Error::Message(format!("invalid FiniteBasisCorr: {n}"))),
        }
    }
}

impl FromKeyValue for FiniteBasisCorr {
    const KEY_NAME: &'static str = "FINITE_BASIS_CORR";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for FiniteBasisCorr {
    fn to_cell(&self) -> Cell {
        let value = match self {
            FiniteBasisCorr::None => 0,
            FiniteBasisCorr::Manual => 1,
            FiniteBasisCorr::Automatic => 2,
        };
        Cell::KeyValue("FINITE_BASIS_CORR", CellValue::Int(value))
    }
}

impl ToCellValue for FiniteBasisCorr {
    fn to_cell_value(&self) -> CellValue {
        let value = match self {
            FiniteBasisCorr::None => 0,
            FiniteBasisCorr::Manual => 1,
            FiniteBasisCorr::Automatic => 2,
        };
        CellValue::Int(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::CellValue;

    #[test]
    fn test_all_variants() {
        assert_eq!(FiniteBasisCorr::from_cell_value(&CellValue::Int(0)).unwrap(), FiniteBasisCorr::None);
        assert_eq!(FiniteBasisCorr::from_cell_value(&CellValue::Int(1)).unwrap(), FiniteBasisCorr::Manual);
        assert_eq!(FiniteBasisCorr::from_cell_value(&CellValue::Int(2)).unwrap(), FiniteBasisCorr::Automatic);
    }

    #[test]
    fn test_invalid() {
        assert!(FiniteBasisCorr::from_cell_value(&CellValue::Int(3)).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(FiniteBasisCorr::KEY_NAME, "FINITE_BASIS_CORR");
    }
}

