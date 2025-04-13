use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::{span_into_str, Rule};

#[derive(
    Debug,
    Clone,
    Hash,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(borrowed_value=str, field="CHECKPOINT", from=String, default_value="test.check".to_string())]
#[pest_ast(rule(Rule::checkpoint))]
#[pest_rule(rule=Rule::checkpoint, keyword="CHECKPOINT")]
/// This keyword contains a string which specifies the name of file to which checkpoint (continuation) data are to be written.
/// # Default
/// `seedname.check`
/// # Example
/// `CHECKPOINT : test.check`
pub struct Checkpoint(
    #[pest_ast(inner(rule(Rule::checkpoint_file), with(span_into_str), with(String::from)))] String,
);
