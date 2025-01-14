use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

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
    Default,
    KeywordDisplay,
)]
#[keyword_display(field="ELEC_RESTORE_FILE",from=String,borrowed_value=str)]
/// This keyword specifies the name of the file from which wavefunction and density data should be restored when performing a CONTINUATION or a REUSE run.
/// NULL means that wavefunction and density data will not be restored.
/// # Note
/// The basis set and distribution for the new run must be the same as those from the run in which the data file was written.
/// # Default
/// NULL
/// # Example
/// `ELEC_RESTORE_FILE : test.wvfn`
pub struct ElecRestoreFile(String);
