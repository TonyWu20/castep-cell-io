use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the number of electronic iterations for which the total spin is fixed.
///
/// Keyword type: Integer
///
/// Default: 10
///
/// Example:
/// SPIN_FIX : 5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "SPIN_FIX")]
pub struct SpinFix(pub i32);

impl Default for SpinFix {
    fn default() -> Self {
        Self(10) // Default is 10
    }
}

impl ToCell for SpinFix {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SPIN_FIX", CellValue::Int(self.0))
    }
}

impl ToCellValue for SpinFix {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Int(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_spin_fix_serde() {
        let spin_fix_str = "SPIN_FIX : 5";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithSpinFix {
            spin_fix: SpinFix,
        }

        let cell_file_result: Result<CellFileWithSpinFix, _> = from_str(spin_fix_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert_eq!(cell_file.spin_fix.0, 5);

        let spin_fix_instance = SpinFix(-1);
        let serialized_result = to_string(&spin_fix_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized SPIN_FIX (-1): {serialized_string}");
        assert!(serialized_string.contains("SPIN_FIX"));
        assert!(serialized_string.contains("-1"));

        assert_eq!(SpinFix::default(), SpinFix(10));
    }
}
