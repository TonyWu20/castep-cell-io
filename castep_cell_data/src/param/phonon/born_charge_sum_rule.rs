use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies whether to correct the Born effective charge tensor to enforce the sum rule.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// BORN_CHARGE_SUM_RULE : TRUE
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BORN_CHARGE_SUM_RULE")]
pub struct BornChargeSumRule(pub bool);

impl ToCell for BornChargeSumRule {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BORN_CHARGE_SUM_RULE", CellValue::Bool(self.0))
    }
}

impl ToCellValue for BornChargeSumRule {
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
    fn test_born_charge_sum_rule_serde() {
        // 1. Test Deserialization TRUE
        let born_charge_sum_rule_true_str = "BORN_CHARGE_SUM_RULE : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithBornChargeSumRuleTrue {
            born_charge_sum_rule: BornChargeSumRule,
        }

        let cell_file_true_result: Result<CellFileWithBornChargeSumRuleTrue, _> =
            from_str(born_charge_sum_rule_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.born_charge_sum_rule.0);

        // 2. Test Deserialization FALSE
        let born_charge_sum_rule_false_str = "BORN_CHARGE_SUM_RULE : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithBornChargeSumRuleFalse {
            born_charge_sum_rule: BornChargeSumRule,
        }

        let cell_file_false_result: Result<CellFileWithBornChargeSumRuleFalse, _> =
            from_str(born_charge_sum_rule_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.born_charge_sum_rule.0);

        // 3. Test Serialization using ToCell
        let born_charge_sum_rule_instance = BornChargeSumRule(true);
        let serialized_result = to_string(&born_charge_sum_rule_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized BORN_CHARGE_SUM_RULE (TRUE): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("BORN_CHARGE_SUM_RULE"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE"));

        // 4. Test Default
        assert_eq!(BornChargeSumRule::default(), BornChargeSumRule(false));
    }
}
