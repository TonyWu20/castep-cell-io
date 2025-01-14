use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, KeywordDisplay,
)]
#[keyword_display(field="PRINT_MEMORY_USAGE",from=bool,value=bool, default_value=true)]
pub struct PrintMemoryUsage(bool);
