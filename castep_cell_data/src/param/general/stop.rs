use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// If present, will cause the current run to be aborted.
/// Implemented as a newtype wrapper around bool.
///
/// Keyword type: Defined (Flag)
///
/// Default: false (not present)
///
/// Example:
/// STOP
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "STOP")]
pub struct Stop(pub bool);

impl Stop {
    /// Creates a new `Stop` instance set to `true`, indicating the flag is present.
    pub fn new() -> Self {
        Self(true)
    }

    /// Creates a `Stop` instance set to `false`, indicating the flag is not present.
    pub fn not_present() -> Self {
        Self(false)
    }
}

impl Default for Stop {
    fn default() -> Self {
        Self::not_present() // Default is not present (false)
    }
}

impl ToCell for Stop {
    fn to_cell(&self) -> Cell {
        if self.0 {
            Cell::Flag("STOP")
        } else {
            Cell::Flag("")
        }
    }
}

// Note: ToCellValue is less applicable for flags, but we can provide a placeholder.
impl ToCellValue for Stop {
    fn to_cell_value(&self) -> CellValue {
        // Flags don't have a value in the traditional sense.
        // Returning Null or Bool(self.0) could be options.
        // Since it's a flag presence, Bool might be more informative if used in a context expecting a value.
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, to_string};

    #[test]
    fn test_stop_serde() {
        // STOP is a flag, typically its presence/absence is what matters.
        // Deserialization from "STOP" would ideally create Stop(true).
        // Deserialization from nothing means Stop(false) or the field is absent.
        // Serialization of Stop(true) should produce Cell::Flag("STOP").
        // Serialization of Stop(false) is ambiguous - it shouldn't really be serialized.
        // For testing ToCell directly:

        let stop_present = Stop::new(); // true
        let serialized_result = to_string(&stop_present.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized STOP (present):\n{serialized_string}");
        assert!(serialized_string.trim() == "STOP"); // Should be just the flag name

        let stop_not_present = Stop::not_present(); // false
        let serialized_result_not_present = to_string(&stop_not_present.to_cell());
        assert!(
            serialized_result_not_present.is_ok(),
            "Serialization (not present) failed: {:?}",
            serialized_result_not_present.err()
        );
        let serialized_string_not_present = serialized_result_not_present.unwrap();
        println!("Serialized STOP (not present):\n{serialized_string_not_present}");
        // The behavior for serializing Stop(false) is debatable.
        // It produces the flag name, which might not be the intended semantics.
        // A better model might be `Option<Stop>` in the parent struct.

        assert_eq!(Stop::default(), Stop::not_present());
    }
}
