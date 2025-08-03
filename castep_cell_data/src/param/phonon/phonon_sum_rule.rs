use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies whether to correct the dynamical matrix to enforce the acoustic sum rule.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// PHONON_SUM_RULE : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename = "PHONON_SUM_RULE")]
pub struct PhononSumRule(pub bool);

impl ToCell for PhononSumRule {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PHONON_SUM_RULE", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PhononSumRule {
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
    fn test_phonon_sum_rule_serde() {
        // 1. Test Deserialization TRUE
        let phonon_sum_rule_true_str = "PHONON_SUM_RULE : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPhononSumRuleTrue {
            phonon_sum_rule: PhononSumRule,
        }

        let cell_file_true_result: Result<CellFileWithPhononSumRuleTrue, _> =
            from_str(phonon_sum_rule_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.phonon_sum_rule.0);

        // 2. Test Deserialization FALSE
        let phonon_sum_rule_false_str = "PHONON_SUM_RULE : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPhononSumRuleFalse {
            phonon_sum_rule: PhononSumRule,
        }

        let cell_file_false_result: Result<CellFileWithPhononSumRuleFalse, _> =
            from_str(phonon_sum_rule_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.phonon_sum_rule.0);

        // 3. Test Serialization using ToCell
        let phonon_sum_rule_instance = PhononSumRule(true);
        let serialized_result = to_string(&phonon_sum_rule_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized PHONON_SUM_RULE (TRUE): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("PHONON_SUM_RULE"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE"));

        // 4. Test Default
        assert_eq!(PhononSumRule::default(), PhononSumRule(false));
    }
}
