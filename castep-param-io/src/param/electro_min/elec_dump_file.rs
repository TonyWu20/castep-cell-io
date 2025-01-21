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
#[keyword_display(field="ELEC_DUMP_FILE",from=String,borrowed_value=str)]
/// This keyword determines the name of the file into which wavefunction and
/// density data are written, periodically during electronic minimization. This file can be used as a backup and is restored with the ELEC_RESTORE_FILE parameter.
/// # Note
/// If this parameter is set to NULL, no backup wavefunction or density data will be written.
/// # Default
/// `seedname.wvfn`
/// # Example
/// `ELEC_DUMP_FILE : test.wvfn`
pub struct ElecDumpFile(String);
