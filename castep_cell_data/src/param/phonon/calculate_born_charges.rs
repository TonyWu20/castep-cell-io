use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies whether to compute Born effective charge tensors.
///
/// Keyword type: Logical
///
/// Default: true
///
/// Example:
/// CALCULATE_BORN_CHARGES : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CALCULATE_BORN_CHARGES")]
pub struct CalculateBornCharges(pub bool);

impl Default for CalculateBornCharges {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl ToCell for CalculateBornCharges {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("CALCULATE_BORN_CHARGES", CellValue::Bool(self.0))
    }
}

impl ToCellValue for CalculateBornCharges {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_calculate_born_charges_serde() {
        // 1. Test Deserialization TRUE
        let calculate_born_charges_true_str = "CALCULATE_BORN_CHARGES : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithCalculateBornChargesTrue {
            calculate_born_charges: CalculateBornCharges,
        }

        let cell_file_true_result: Result<CellFileWithCalculateBornChargesTrue, _> =
            from_str(calculate_born_charges_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.calculate_born_charges.0);

        // 2. Test Deserialization FALSE
        let calculate_born_charges_false_str = "CALCULATE_BORN_CHARGES : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithCalculateBornChargesFalse {
            calculate_born_charges: CalculateBornCharges,
        }

        let cell_file_false_result: Result<CellFileWithCalculateBornChargesFalse, _> =
            from_str(calculate_born_charges_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.calculate_born_charges.0);

        // 3. Test Serialization using ToCell
        let calculate_born_charges_instance = CalculateBornCharges(false);
        let serialized_result = to_string(&calculate_born_charges_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized CALCULATE_BORN_CHARGES (FALSE): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("CALCULATE_BORN_CHARGES"));
        assert!(serialized_string.contains("false") || serialized_string.contains("FALSE"));

        // 4. Test Default
        assert_eq!(CalculateBornCharges::default(), CalculateBornCharges(true));
    }
}
