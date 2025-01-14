use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, KeywordDisplay,
)]
#[keyword_display(field="NLXC_EXCHANGE_REFLECT_KPTS", from=bool,value=bool, default_value= true)]
pub struct ExchangeReflectKpts(bool);
