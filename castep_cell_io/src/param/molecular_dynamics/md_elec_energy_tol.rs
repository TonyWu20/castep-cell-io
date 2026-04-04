use crate::units::EnergyUnit;
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::row_as_f64_n;

/// Controls the tolerance for accepting convergence of the total energy during MD.
///
/// Keyword type: Real
///
/// Default: Same as ELEC_ENERGY_TOL
///
/// Example:
/// MD_ELEC_ENERGY_TOL : 0.00007 eV
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MdElecEnergyTol {
    /// The energy tolerance value.
    pub value: f64,
    /// The optional unit of the energy value.
    pub unit: Option<EnergyUnit>,
}

impl FromCellValue for MdElecEnergyTol {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(_) => {
                let arr = row_as_f64_n::<2>(value)?;
                Ok(Self {
                    value: arr[0],
                    unit: if arr[1] > 0.0 {
                        Some(EnergyUnit::from_cell_value(&CellValue::Float(arr[1]))?)
                    } else {
                        None
                    },
                })
            }
            CellValue::Float(f) => Ok(Self {
                value: *f,
                unit: None,
            }),
            _ => Err(Error::Message("expected float or array".to_string())),
        }
    }
}

impl FromKeyValue for MdElecEnergyTol {
    const KEY_NAME: &'static str = "MD_ELEC_ENERGY_TOL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdElecEnergyTol {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_ELEC_ENERGY_TOL", self.to_cell_value())
    }
}

impl ToCellValue for MdElecEnergyTol {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_float_only() {
        let val = CellValue::Float(0.00007);
        let result = MdElecEnergyTol::from_cell_value(&val).unwrap();
        assert_eq!(result.value, 0.00007);
        assert!(result.unit.is_none());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MdElecEnergyTol::KEY_NAME, "MD_ELEC_ENERGY_TOL");
    }
}

