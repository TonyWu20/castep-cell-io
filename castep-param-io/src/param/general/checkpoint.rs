use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, KeywordDisplay,
)]
#[keyword_display(borrowed_value=str, field="CHECKPOINT", from=String, default_value="test.check".to_string())]
/// This keyword contains a string which specifies the name of file to which checkpoint (continuation) data are to be written.
/// # Default
/// `seedname.check`
/// # Example
/// `CHECKPOINT : test.check`
pub struct Checkpoint(String);
