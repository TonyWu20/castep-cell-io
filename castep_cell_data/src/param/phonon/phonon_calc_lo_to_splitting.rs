use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies whether to calculate the non-analytical contribution for LO/TO splitting.
///
/// Keyword type: Logical
///
/// Default: true
///
/// Example:
/// PHONON_CALC_LO_TO_SPLITTING : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PHONON_CALC_LO_TO_SPLITTING")]
pub struct PhononCalcLoToSplitting(pub bool);

impl Default for PhononCalcLoToSplitting {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl ToCell for PhononCalcLoToSplitting {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_CALC_LO_TO_SPLITTING", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PhononCalcLoToSplitting {
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
    fn test_phonon_calc_lo_to_splitting_serde() {
        // 1. Test Deserialization TRUE
        let phonon_calc_lo_to_splitting_true_str = "PHONON_CALC_LO_TO_SPLITTING : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPhononCalcLoToSplittingTrue {
            phonon_calc_lo_to_splitting: PhononCalcLoToSplitting,
        }

        let cell_file_true_result: Result<CellFileWithPhononCalcLoToSplittingTrue, _> =
            from_str(phonon_calc_lo_to_splitting_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.phonon_calc_lo_to_splitting.0);

        // 2. Test Deserialization FALSE
        let phonon_calc_lo_to_splitting_false_str = "PHONON_CALC_LO_TO_SPLITTING : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPhononCalcLoToSplittingFalse {
            phonon_calc_lo_to_splitting: PhononCalcLoToSplitting,
        }

        let cell_file_false_result: Result<CellFileWithPhononCalcLoToSplittingFalse, _> =
            from_str(phonon_calc_lo_to_splitting_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.phonon_calc_lo_to_splitting.0);

        // 3. Test Serialization using ToCell
        let phonon_calc_lo_to_splitting_instance = PhononCalcLoToSplitting(false);
        let serialized_result = to_string(&phonon_calc_lo_to_splitting_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized PHONON_CALC_LO_TO_SPLITTING (FALSE): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("PHONON_CALC_LO_TO_SPLITTING"));
        assert!(serialized_string.contains("false") || serialized_string.contains("FALSE"));

        // 4. Test Default
        assert_eq!(
            PhononCalcLoToSplitting::default(),
            PhononCalcLoToSplitting(true)
        );
    }
}
