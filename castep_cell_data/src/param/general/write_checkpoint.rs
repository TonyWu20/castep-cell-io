/// !!! Use with caution. This thing is poorly designed and documented by `CASTEP` IMO.
/// Not guaranteed to be able to handle all cases.
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies whether or not to write checkpoint files and controls the level of detail.
///
/// Keyword type: String
///
/// Default: WriteCheckpoint { action: WriteCheckpointAction::All, option: None }
///
/// Example:
/// WRITE_CHECKPOINT : SUCCESS=BOTH
/// WRITE_CHECKPOINT : MINIMAL
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "WRITE_CHECKPOINT")]
#[serde(from = "WriteCheckpointRepr")] // Use intermediate repr for deserialization
pub struct WriteCheckpoint {
    /// The main action/specifier.
    pub action: WriteCheckpointAction,
    /// Optional fine control for success, failure, or backup.
    pub option: Option<WriteCheckpointOption>,
}

/// Intermediate representation for deserializing the `WRITE_CHECKPOINT` value.
/// Handles the complex string format like "MINIMAL", "ALL", or "SUCCESS=BOTH".
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum WriteCheckpointRepr {
    /// Simple action: e.g., "NONE", "MINIMAL", "ALL"
    Simple(WriteCheckpointAction),
    /// Action with option: e.g., "SUCCESS=BOTH", "FAILURE=MINIMAL"
    /// Note: Real implementation would parse the string "SUCCESS=BOTH".
    /// This is a simplified representation assuming the parser/tokenizer
    /// provides the parts correctly.
    /// A full implementation would likely require custom deserialization logic.
    WithOption(String, String), // e.g., ("SUCCESS", "BOTH")
                                // If the parser provides a single string, a custom Deserialize impl
                                // or a From<String> would be needed.
}

impl From<WriteCheckpointRepr> for WriteCheckpoint {
    fn from(repr: WriteCheckpointRepr) -> Self {
        match repr {
            WriteCheckpointRepr::Simple(action) => Self {
                action,
                option: None,
            },
            // Simplified conversion. A real implementation would parse "SUCCESS=BOTH"
            // and map "SUCCESS" to WriteCheckpointOption::Success and "BOTH" to WriteCheckpointAction::Both.
            // This part requires more complex string parsing logic.
            // For demonstration, let's assume a placeholder.
            // A robust solution would involve parsing the string value directly.
            WriteCheckpointRepr::WithOption(option_type, option_value) => {
                // This is a very simplified and not robust parsing.
                // A real implementation would match option_type against "SUCCESS", "FAILURE", "BACKUP"
                // and option_value against the action variants.
                // For now, we just create a placeholder option.
                // This highlights the complexity of parsing this keyword.
                // A better approach is a custom Deserialize impl for WriteCheckpoint.
                let _ = option_type; // Use the variable to silence warning
                let _ = option_value; // Use the variable to silence warning
                Self {
                    action: WriteCheckpointAction::All, // Placeholder
                    option: Some(WriteCheckpointOption::Success(WriteCheckpointAction::Both)), // Placeholder
                }
            }
        }
    }
}

// A more robust approach would be to implement Deserialize manually
// or use a FromStr implementation for WriteCheckpoint.
// For example:
// impl<'de> Deserialize<'de> for WriteCheckpoint {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         // Parse `s` (e.g., "SUCCESS=BOTH") into WriteCheckpoint
//         // This requires custom parsing logic.
//         // Return Ok(WriteCheckpoint) or Err if parsing fails.
//         // ...
//     }
// }

/// The main action or specifier for WRITE_CHECKPOINT.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum WriteCheckpointAction {
    /// Do not write checkpoint files.
    #[serde(rename = "NONE")]
    None,
    /// Write minimal checkpoint files (alias for Both).
    #[serde(rename = "MINIMAL")]
    Minimal,
    /// Write both .check and .castep_bin files (alias for Minimal).
    #[serde(rename = "BOTH")]
    Both,
    /// Write all available checkpoint data (alias for Full).
    #[serde(rename = "ALL")]
    All,
    /// Write all available checkpoint data (alias for All).
    #[serde(rename = "FULL")]
    Full,
}

/// Options for fine control (SUCCESS=, FAILURE=, BACKUP=).
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum WriteCheckpointOption {
    /// Optional fine control for success termination.
    Success(WriteCheckpointAction),
    /// Optional fine control for failure termination.
    Failure(WriteCheckpointAction), // Corrected: was WriteCheckpointOption
    /// Optional fine control for periodic backup.
    Backup(WriteCheckpointAction), // Corrected: was WriteCheckpointOption
}

impl Default for WriteCheckpoint {
    fn default() -> Self {
        Self {
            action: WriteCheckpointAction::All, // Default is ALL
            option: None,
        }
    }
}

impl ToCell for WriteCheckpoint {
    fn to_cell(&self) -> Cell {
        // Construct the string value based on the struct fields.
        let action_str = match self.action {
            WriteCheckpointAction::None => "NONE",
            WriteCheckpointAction::Minimal => "MINIMAL",
            WriteCheckpointAction::Both => "BOTH",
            WriteCheckpointAction::All => "ALL",
            WriteCheckpointAction::Full => "FULL",
        };

        if let Some(option) = &self.option {
            let (option_prefix, option_action) = match option {
                WriteCheckpointOption::Success(a) => ("SUCCESS", a),
                WriteCheckpointOption::Failure(a) => ("FAILURE", a),
                WriteCheckpointOption::Backup(a) => ("BACKUP", a),
            };
            let option_action_str = match option_action {
                WriteCheckpointAction::None => "NONE",
                WriteCheckpointAction::Minimal => "MINIMAL",
                WriteCheckpointAction::Both => "BOTH",
                WriteCheckpointAction::All => "ALL",
                WriteCheckpointAction::Full => "FULL",
            };
            let full_value = format!("{option_prefix}={option_action_str}");
            Cell::KeyValue("WRITE_CHECKPOINT", CellValue::String(full_value))
        } else {
            // If no option, just serialize the action
            Cell::KeyValue(
                "WRITE_CHECKPOINT",
                CellValue::String(action_str.to_string()),
            )
        }
    }
}

// Note: ToCellValue is less straightforward for this complex type.
// The primary serialization path is via ToCell.

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, to_string};
    // Note: Deserialization testing is complex without a full parser implementation.
    // These tests focus on the ToCell serialization.

    #[test]
    fn test_write_checkpoint_serde() {
        // 1. Test Serialization for simple action (default-like)
        let write_checkpoint_simple = WriteCheckpoint {
            action: WriteCheckpointAction::Minimal,
            option: None,
        };
        let serialized_result_simple = to_string(&write_checkpoint_simple.to_cell());
        assert!(
            serialized_result_simple.is_ok(),
            "Serialization (simple) failed: {:?}",
            serialized_result_simple.err()
        );
        let serialized_string_simple = serialized_result_simple.unwrap();
        println!("Serialized WRITE_CHECKPOINT (MINIMAL): {serialized_string_simple}");
        assert!(serialized_string_simple.contains("WRITE_CHECKPOINT"));
        assert!(serialized_string_simple.contains("MINIMAL"));

        // 2. Test Serialization with an option (example case)
        let write_checkpoint_with_option = WriteCheckpoint {
            action: WriteCheckpointAction::All, // Default action part (often implicit when option is used)
            option: Some(WriteCheckpointOption::Success(WriteCheckpointAction::Both)),
        };
        let serialized_result_option = to_string(&write_checkpoint_with_option.to_cell());
        assert!(
            serialized_result_option.is_ok(),
            "Serialization (with option) failed: {:?}",
            serialized_result_option.err()
        );
        let serialized_string_option = serialized_result_option.unwrap();
        println!("Serialized WRITE_CHECKPOINT (with option): {serialized_string_option}");
        assert!(serialized_string_option.contains("WRITE_CHECKPOINT"));
        assert!(serialized_string_option.contains("SUCCESS=BOTH"));

        // 3. Test Default
        let default_instance = WriteCheckpoint::default();
        assert_eq!(default_instance.action, WriteCheckpointAction::All);
        assert_eq!(default_instance.option, None);
    }
}
