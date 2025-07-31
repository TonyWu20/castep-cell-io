use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming EnergyUnit exists in units module
use crate::units::EnergyUnit;

/// Determines the spacing of cutoff energies used to estimate the BASIS_DE_DLOGE
/// in automatic calculation of the finite basis set correction.
///
/// Keyword type: Real
///
/// Default: 5.0 eV
///
/// Example:
/// FINITE_BASIS_SPACING : 0.2 Ha
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "FINITE_BASIS_SPACING")]
pub struct FiniteBasisSpacing {
    /// The spacing value.
    pub value: f64,
    /// The unit of the energy value.
    pub unit: EnergyUnit,
}

// Intermediate representation for deserialization
#[derive(Debug, Deserialize)]
struct FiniteBasisSpacingRepr {
    value: f64,
    unit: EnergyUnit,
}

impl From<FiniteBasisSpacingRepr> for FiniteBasisSpacing {
    fn from(repr: FiniteBasisSpacingRepr) -> Self {
        Self {
            value: repr.value,
            unit: repr.unit,
        }
    }
}

impl ToCell for FiniteBasisSpacing {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue(
            "FINITE_BASIS_SPACING",
            CellValue::Array(vec![
                CellValue::Float(self.value),
                self.unit.to_cell_value(),
            ]),
        )
    }
}

impl ToCellValue for FiniteBasisSpacing {
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
    fn test_finite_basis_spacing_serde() {
        let finite_basis_spacing_str = "FINITE_BASIS_SPACING : 0.2 ha";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFiniteBasisSpacing {
            finite_basis_spacing: FiniteBasisSpacing,
        }

        let cell_file_result: Result<CellFileWithFiniteBasisSpacing, _> =
            from_str(finite_basis_spacing_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.finite_basis_spacing.value - 0.2).abs() < 1e-10);
        assert_eq!(cell_file.finite_basis_spacing.unit, EnergyUnit::Hartree);

        let finite_basis_spacing_instance = FiniteBasisSpacing {
            value: 1.0,
            unit: EnergyUnit::ElectronVolt,
        };
        let serialized_result = to_string(&finite_basis_spacing_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized FINITE_BASIS_SPACING (1.0 ev): {serialized_string}");
        assert!(serialized_string.contains("FINITE_BASIS_SPACING"));
        assert!(serialized_string.contains("1.0"));
        assert!(serialized_string.contains("ev"));
    }
}
