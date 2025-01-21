use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field="PRINT_CLOCK",from=bool,value=bool, default_value=true)]
pub struct PrintClock(bool);
