use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies whether or not timing information will be printed.
///
/// Keyword type: Logical
///
/// Default: TRUE
///
/// Example:
/// PRINT_CLOCK : TRUE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PRINT_CLOCK")]
pub struct PrintClock(pub bool);

impl Default for PrintClock {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl ToCell for PrintClock {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("PRINT_CLOCK", CellValue::Bool(self.0))
    }
}

impl ToCellValue for PrintClock {
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
    fn test_print_clock_serde() {
        let print_clock_true_str = "PRINT_CLOCK : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPrintClockTrue {
            print_clock: PrintClock,
        }

        let cell_file_true_result: Result<CellFileWithPrintClockTrue, _> =
            from_str(print_clock_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.print_clock.0);

        let print_clock_false_str = "PRINT_CLOCK : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithPrintClockFalse {
            print_clock: PrintClock,
        }

        let cell_file_false_result: Result<CellFileWithPrintClockFalse, _> =
            from_str(print_clock_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.print_clock.0);

        let print_clock_instance = PrintClock(false);
        let serialized_result = to_string(&print_clock_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized PRINT_CLOCK (FALSE):\n{serialized_string}");
        assert!(serialized_string.contains("PRINT_CLOCK"));
        assert!(serialized_string.contains("false") || serialized_string.contains("FALSE"));

        assert_eq!(PrintClock::default(), PrintClock(true));
    }
}
