use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(specified_fields = true)]
pub enum BackUpSetting {
    /// This keyword specifies the interval, in seconds, between updates of the
    /// backup restart files. This keyword is applicable for geometry optimization,
    /// molecular dynamics, phonon or transition state search runs.
    /// A value which is less than or equal to zero indicates that no updates will be performed.
    #[keyword_display(field = "BACKUP_INTERVAL")]
    BackupInterval(i64),
    // This keyword specifies the number of geometry optimization or molecular
    // dynamics iterations between updates of the backup restart files.
    #[keyword_display(field = "NUM_BACKUP_ITER")]
    NumBackupIter(u64),
}

impl Default for BackUpSetting {
    fn default() -> Self {
        Self::NumBackupIter(5)
    }
}
