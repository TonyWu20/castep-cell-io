use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls whether or not best fit extrapolation parameters will be calculated.
///
/// Keyword type: Logical
///
/// Default: true
///
/// Example:
/// MD_EXTRAP_FIT = FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MD_EXTRAP_FIT")]
pub struct MdExtrapFit(pub bool);

impl Default for MdExtrapFit {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl ToCell for MdExtrapFit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_EXTRAP_FIT", CellValue::Bool(self.0))
    }
}

impl ToCellValue for MdExtrapFit {
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
    fn test_md_extrap_fit_serde() {
        // 1. Test Deserialization TRUE
        // Note: The example uses '=' instead of ':'. Assuming ':' is the standard.
        let md_extrap_fit_true_str = "MD_EXTRAP_FIT : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdExtrapFitTrue {
            md_extrap_fit: MdExtrapFit,
        }

        let cell_file_true_result: Result<CellFileWithMdExtrapFitTrue, _> =
            from_str(md_extrap_fit_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.md_extrap_fit.0);

        // 2. Test Deserialization FALSE
        let md_extrap_fit_false_str = "MD_EXTRAP_FIT : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdExtrapFitFalse {
            md_extrap_fit: MdExtrapFit,
        }

        let cell_file_false_result: Result<CellFileWithMdExtrapFitFalse, _> =
            from_str(md_extrap_fit_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.md_extrap_fit.0);

        // 3. Test Serialization using ToCell
        let md_extrap_fit_instance = MdExtrapFit(false);
        let serialized_result = to_string(&md_extrap_fit_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MD_EXTRAP_FIT (FALSE): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("MD_EXTRAP_FIT"));
        assert!(serialized_string.contains("false") || serialized_string.contains("FALSE"));

        // 4. Test Default
        assert_eq!(MdExtrapFit::default(), MdExtrapFit(true));
    }
}
