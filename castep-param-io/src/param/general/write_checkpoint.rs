use std::fmt::Display;

use castep_param_derive::KeywordDisplay;
use pest::Span;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::Rule;

#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    KeywordDisplay,
    FromPest,
)]
#[keyword_display(field = "WRITE_CHECKPOINT")]
#[pest_ast(rule(Rule::write_checkpoint))]
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
    Debug,
    Clone,
    Copy,
    Default,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    FromPest,
)]
#[pest_ast(rule(Rule::write_checkpoint_value))]
pub enum WriteCheckpointValue {
    #[pest_ast(outer(with(span_into_write_checkpoint_value)))]
    None,
    Minimal,
    Both,
    #[default]
    All,
    Full,
}

fn span_into_write_checkpoint_value(span: Span<'_>) -> WriteCheckpointValue {
    let value = span.as_str().to_lowercase();
    match value.as_str() {
        "none" => Some(WriteCheckpointValue::None),
        "minimal" => Some(WriteCheckpointValue::Minimal),
        "both" => Some(WriteCheckpointValue::Both),
        "all" => Some(WriteCheckpointValue::All),
        "full" => Some(WriteCheckpointValue::Full),
        _ => None,
    }
    .expect("Always correct from parsed result.")
}

impl Display for WriteCheckpointValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteCheckpointValue::None => f.write_str("None"),
            WriteCheckpointValue::Minimal => f.write_str("Minimal"),
            WriteCheckpointValue::Both => f.write_str("Both"),
            WriteCheckpointValue::All => f.write_str("All"),
            WriteCheckpointValue::Full => f.write_str("Full"),
        }
    }
}

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, FromPest,
)]
#[pest_ast(rule(Rule::write_checkpoint_option))]
pub enum WriteCheckpointOption {
    Success(WriteCheckpointValue),
    Failure(WriteCheckpointValue),
    Backup(WriteCheckpointValue),
}

// fn span_into_write_checkpoint_option(span:Span<'_>) -> WriteCheckpointOption

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
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{
        param::{general::write_checkpoint::WriteCheckpointOption, KeywordDisplay},
        parser::{ParamParser, Rule},
    };

    use super::WriteCheckpoint;

    #[test]
    fn write_checkpoint() {
        let write_checkpoint = WriteCheckpoint::default();
        assert_eq!("WRITE_CHECKPOINT : All", write_checkpoint.output());
        let write_checkpoint_option = WriteCheckpoint::Option(WriteCheckpointOption::default());
        assert_eq!(
            "WRITE_CHECKPOINT : BACKUP=Minimal",
            write_checkpoint_option.output()
        );
        let binding = write_checkpoint_option.output();
        let mut parse = ParamParser::parse(Rule::write_checkpoint, &binding).unwrap();
        dbg!(&parse);
        let parsed = WriteCheckpoint::from_pest(&mut parse);
        dbg!(parsed);
    }
}
