use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines whether or not to update the estimate of the Thomas-Fermi screening length.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// BS_RE_EST_K_SCRN : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BS_RE_EST_K_SCRN")]
pub struct BsReEstKScrn(pub bool);

impl ToCell for BsReEstKScrn {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BS_RE_EST_K_SCRN", CellValue::Bool(self.0))
    }
}

impl ToCellValue for BsReEstKScrn {
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
    fn test_bs_re_est_k_scrn_serde() {
        // 1. Test Deserialization TRUE
        let bs_re_est_k_scrn_true_str = "BS_RE_EST_K_SCRN : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithBsReEstKScrnTrue {
            bs_re_est_k_scrn: BsReEstKScrn,
        }

        let cell_file_true_result: Result<CellFileWithBsReEstKScrnTrue, _> =
            from_str(bs_re_est_k_scrn_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.bs_re_est_k_scrn.0);

        // 2. Test Deserialization FALSE
        let bs_re_est_k_scrn_false_str = "BS_RE_EST_K_SCRN : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithBsReEstKScrnFalse {
            bs_re_est_k_scrn: BsReEstKScrn,
        }

        let cell_file_false_result: Result<CellFileWithBsReEstKScrnFalse, _> =
            from_str(bs_re_est_k_scrn_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.bs_re_est_k_scrn.0);

        // 3. Test Serialization using ToCell
        let bs_re_est_k_scrn_instance = BsReEstKScrn(true);
        let serialized_result = to_string(&bs_re_est_k_scrn_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized BS_RE_EST_K_SCRN (TRUE): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("BS_RE_EST_K_SCRN"));
        assert!(serialized_string.contains("true") || serialized_string.contains("TRUE"));

        // 4. Test Default
        assert_eq!(BsReEstKScrn::default(), BsReEstKScrn(false));
    }
}
