use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, KeywordDisplay,
)]
#[keyword_display(field = "GEOM_MAX_ITER", from=u32,value=u32, default_value=30)]
/// This keyword determines the maximum number of steps in a geometry optimization.
/// # Default
/// 30
/// # Example
/// `GEOM_MAX_ITER : 25`
pub struct GeomMaxIter(u32);
