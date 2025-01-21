use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};
/// This keyword specifies the number of SCF cycles between updates to the
/// wavefunction and density data file. If NUM_DUMP_CYCLES is less than or equal
/// to 0, no data will be written to this file.
/// # Default
/// 5
/// # Example
/// `NUM_DUMP_CYCLES : 10`
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field="NUM_DUMP_CYCLES", from=u32,value= u32)]
pub struct NumDumpCycles(u32);
