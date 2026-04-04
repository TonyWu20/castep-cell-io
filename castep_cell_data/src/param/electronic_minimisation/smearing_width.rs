use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_io::query::*;
use castep_cell_io::parse::FromCellValue;
use crate::units::EnergyUnit;

/// Determines the width of the Fermi-surface smearing.
///
/// Keyword type: Real
///
/// Default: 0.2 eV
///
/// Example:
/// SMEARING_WIDTH : 0.1 eV
/// SMEARING_WIDTH : 0.1 (uses default unit, likely eV)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SmearingWidth {
    /// The smearing width value.
    pub value: f64,
    /// The optional unit of the energy value.
    /// If None, a default unit (likely eV) is implied.
    pub unit: Option<EnergyUnit>,
}

impl FromKeyValue for SmearingWidth {
    const KEY_NAME: &'static str = "SMEARING_WIDTH";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if !arr.is_empty() => {
                let val = value_as_f64(&arr[0])?;
                let unit = if arr.len() > 1 {
                    Some(EnergyUnit::from_cell_value(&arr[1])?)
                } else {
                    None
                };
                Ok(Self { value: val, unit })
            }
            _ => {
                let val = value_as_f64(value)?;
                Ok(Self { value: val, unit: None })
            }
        }
    }
}

impl ToCell for SmearingWidth {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SMEARING_WIDTH", self.to_cell_value())
    }
}

impl ToCellValue for SmearingWidth {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            [
                CellValue::Float(self.value),
                self.unit
                    .as_ref()
                    .map(|u| u.to_cell_value())
                    .unwrap_or(CellValue::Null),
            ]
            .to_vec(),
        )
    }
}


