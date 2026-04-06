/// !!! Use with caution. This thing is poorly designed and documented by `CASTEP` IMO.
/// Not guaranteed to be able to handle all cases.
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_fmt::parse::FromKeyValue;
use castep_cell_fmt::query::value_as_str;
use castep_cell_fmt::Error;

/// Specifies whether or not to write checkpoint files and controls the level of detail.
///
/// Keyword type: String
///
/// Default: WriteCheckpoint { action: WriteCheckpointAction::All, option: None }
///
/// Example:
/// WRITE_CHECKPOINT : SUCCESS=BOTH
/// WRITE_CHECKPOINT : MINIMAL
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteCheckpoint {
    /// The main action/specifier.
    pub action: WriteCheckpointAction,
    /// Optional fine control for success, failure, or backup.
    pub option: Option<WriteCheckpointOption>,
}

/// The main action or specifier for WRITE_CHECKPOINT.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WriteCheckpointAction {
    /// Do not write checkpoint files.
    None,
    /// Write minimal checkpoint files (alias for Both).
    Minimal,
    /// Write both .check and .castep_bin files (alias for Minimal).
    Both,
    /// Write all available checkpoint data (alias for Full).
    All,
    /// Write all available checkpoint data (alias for All).
    Full,
}

/// Options for fine control (SUCCESS=, FAILURE=, BACKUP=).
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WriteCheckpointOption {
    /// Optional fine control for success termination.
    Success(WriteCheckpointAction),
    /// Optional fine control for failure termination.
    Failure(WriteCheckpointAction),
    /// Optional fine control for periodic backup.
    Backup(WriteCheckpointAction),
}

impl Default for WriteCheckpoint {
    fn default() -> Self {
        Self {
            action: WriteCheckpointAction::All, // Default is ALL
            option: None,
        }
    }
}

impl FromKeyValue for WriteCheckpoint {
    const KEY_NAME: &'static str = "WRITE_CHECKPOINT";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        let s = value_as_str(value)?;

        if let Some((option_type, option_value)) = s.split_once('=') {
            let action = parse_action(option_value.trim())?;
            let option = match option_type.trim().to_ascii_uppercase().as_str() {
                "SUCCESS" => Some(WriteCheckpointOption::Success(action)),
                "FAILURE" => Some(WriteCheckpointOption::Failure(action)),
                "BACKUP" => Some(WriteCheckpointOption::Backup(action)),
                other => return Err(Error::Message(format!("unknown WriteCheckpoint option: {other}"))),
            };
            Ok(Self {
                action: WriteCheckpointAction::All,
                option,
            })
        } else {
            let action = parse_action(s)?;
            Ok(Self {
                action,
                option: None,
            })
        }
    }
}

fn parse_action(s: &str) -> CResult<WriteCheckpointAction> {
    match s.to_ascii_uppercase().as_str() {
        "NONE" => Ok(WriteCheckpointAction::None),
        "MINIMAL" => Ok(WriteCheckpointAction::Minimal),
        "BOTH" => Ok(WriteCheckpointAction::Both),
        "ALL" => Ok(WriteCheckpointAction::All),
        "FULL" => Ok(WriteCheckpointAction::Full),
        other => Err(Error::Message(format!("unknown WriteCheckpointAction: {other}"))),
    }
}

impl ToCell for WriteCheckpoint {
    fn to_cell(&self) -> Cell<'_> {
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

impl ToCellValue for WriteCheckpoint {
    fn to_cell_value(&self) -> CellValue<'_> {
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
            CellValue::String(format!("{option_prefix}={option_action_str}"))
        } else {
            CellValue::String(action_str.to_string())
        }
    }
}

