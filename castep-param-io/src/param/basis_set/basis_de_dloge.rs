
use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};


#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Default, KeywordDisplay,
)]
#[keyword_display(from=f64, value=f64, field="BASIS_DE_DLOGE")]
pub struct BasisDeDloge(f64);
