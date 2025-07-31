use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};
// Assuming InvLengthUnit exists in units module (1/bohr, 1/ang, etc.)
use crate::units::InvLengthUnit;

/// Determines the maximum size of the g-vectors included in the fine grid.
///
/// Keyword type: Real
///
/// Default: -1 1/bohr (results in fine and standard grids being identical)
///
/// Example:
/// FINE_GMAX : 20 1/ang
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "FINE_GMAX")]
pub struct FineGmax {
    /// The maximum g-vector magnitude value.
    pub value: f64,
    /// The unit of the inverse length value.
    pub unit: InvLengthUnit,
}

// Intermediate representation for deserialization
#[derive(Debug, Deserialize)]
struct FineGmaxRepr {
    value: f64,
    unit: InvLengthUnit,
}

impl From<FineGmaxRepr> for FineGmax {
    fn from(repr: FineGmaxRepr) -> Self {
        Self {
            value: repr.value,
            unit: repr.unit,
        }
    }
}

impl ToCell for FineGmax {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue(
            "FINE_GMAX",
            CellValue::Array(vec![
                CellValue::Float(self.value),
                self.unit.to_cell_value(),
            ]),
        )
    }
}

impl ToCellValue for FineGmax {
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
    fn test_fine_gmax_serde() {
        // Note: The default unit might be 1/bohr. Adjust test if parser handles it differently.
        let fine_gmax_str = "FINE_GMAX : 20 1/ang";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithFineGmax {
            fine_gmax: FineGmax,
        }

        let cell_file_result: Result<CellFileWithFineGmax, _> = from_str(fine_gmax_str);
        // This test might fail if the parser/tokenizer has issues with "1/ang".
        // Ensure your number parser correctly handles unit boundaries.
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.fine_gmax.value - 20.0).abs() < f64::EPSILON);
        // Asserting the unit depends on how InvLengthUnit is defined.
        // assert_eq!(cell_file.fine_gmax.unit, InvLengthUnit::PerAngstrom);

        let fine_gmax_instance = FineGmax {
            value: 15.0,
            unit: InvLengthUnit::Bohr,
        };
        let serialized_result = to_string(&fine_gmax_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized FINE_GMAX (15.0 1/bohr): {serialized_string}");
        assert!(serialized_string.contains("FINE_GMAX"));
        assert!(serialized_string.contains("15.0"));
        // Check for unit string, e.g., "1/bohr"
    }
}
