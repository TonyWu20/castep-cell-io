use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming EnergyUnit exists in units module
use crate::units::EnergyUnit;

/// Specifies the derivative of total energy with respect to the natural log
/// of the basis cutoff energy for manual finite basis set correction.
///
/// Keyword type: Real
///
/// Default: 0.0 (but requires a value if FINITE_BASIS_CORR : 1)
///
/// Example:
/// BASIS_DE_DLOGE : -1.2345 eV
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "BASIS_DE_DLOGE")]
pub struct BasisDeDloge {
    /// The derivative value.
    pub value: f64,
    /// The unit of the energy value.
    pub unit: EnergyUnit,
}

// Intermediate representation for deserialization
#[derive(Debug, Deserialize)]
struct BasisDeDlogeRepr {
    value: f64,
    unit: EnergyUnit,
}

impl From<BasisDeDlogeRepr> for BasisDeDloge {
    fn from(repr: BasisDeDlogeRepr) -> Self {
        Self {
            value: repr.value,
            unit: repr.unit,
        }
    }
}

impl ToCell for BasisDeDloge {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue(
            "BASIS_DE_DLOGE",
            CellValue::Array(vec![
                CellValue::Float(self.value),
                self.unit.to_cell_value(),
            ]),
        )
    }
}

impl ToCellValue for BasisDeDloge {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(vec![
            CellValue::Float(self.value),
            self.unit.to_cell_value(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_basis_de_dloge_serde() {
        let basis_de_dloge_str = "BASIS_DE_DLOGE : -1.2345 ev";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithBasisDeDloge {
            basis_de_dloge: BasisDeDloge,
        }

        let cell_file_result: Result<CellFileWithBasisDeDloge, _> = from_str(basis_de_dloge_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.basis_de_dloge.value - (-1.2345)).abs() < 1e-10);
        assert_eq!(cell_file.basis_de_dloge.unit, EnergyUnit::ElectronVolt);

        let basis_de_dloge_instance = BasisDeDloge {
            value: -0.5,
            unit: EnergyUnit::Hartree,
        };
        let serialized_result = to_string(&basis_de_dloge_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized BASIS_DE_DLOGE (-0.5 ha): {serialized_string}");
        assert!(serialized_string.contains("BASIS_DE_DLOGE"));
        assert!(serialized_string.contains("-0.5"));
        assert!(serialized_string.contains("ha"));
    }
}
