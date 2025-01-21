use std::fmt::Display;

use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};


#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field = "WRITE_CHECKPOINT")]
pub enum WriteCheckpoint {
    Value(WriteCheckpointValue),
    Option(WriteCheckpointOption),
}

impl Default for WriteCheckpoint {
    fn default() -> Self {
        Self::Value(WriteCheckpointValue::All)
    }
}

impl From<WriteCheckpointValue> for WriteCheckpoint {
    fn from(value: WriteCheckpointValue) -> Self {
        Self::Value(value)
    }
}

impl From<WriteCheckpointOption> for WriteCheckpoint {
    fn from(value: WriteCheckpointOption) -> Self {
        Self::Option(value)
    }
}

#[derive(
    Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]

pub enum WriteCheckpointValue {
    None,
    Minimal,
    Both,
    #[default]
    All,
    Full,
}

impl Display for WriteCheckpointValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteCheckpointValue::None => f.write_str("NONE"),
            WriteCheckpointValue::Minimal => f.write_str("MINIMAL"),
            WriteCheckpointValue::Both => f.write_str("bOTH"),
            WriteCheckpointValue::All => f.write_str("ALL"),
            WriteCheckpointValue::Full => f.write_str("fULL"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WriteCheckpointOption {
    Success(WriteCheckpointValue),
    Failure(WriteCheckpointValue),
    Backup(WriteCheckpointValue),
}

impl Default for WriteCheckpointOption {
    fn default() -> Self {
        Self::Backup(WriteCheckpointValue::Minimal)
    }
}

impl Display for WriteCheckpointOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteCheckpointOption::Success(v) => write!(f, "SUCCESS={v}"),
            WriteCheckpointOption::Failure(v) => write!(f, "FAILURE={v}"),
            WriteCheckpointOption::Backup(v) => write!(f, "BACKUP={v}"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::param::{general::write_checkpoint::WriteCheckpointOption, KeywordDisplay};

    use super::WriteCheckpoint;

    #[test]
    fn write_checkpoint() {
        let write_checkpoint = WriteCheckpoint::default();
        assert_eq!("WRITE_CHECKPOINT : ALL", write_checkpoint.output());
        let write_checkpoint_option = WriteCheckpoint::Option(WriteCheckpointOption::default());
        assert_eq!(
            "WRITE_CHECKPOINT : BACKUP=MINIMAL",
            write_checkpoint_option.output()
        );
    }
}
