use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field="GEOM_CONVERGENCE_WIN", from=u32,value=u32, default_value=2)]
/// This keyword determines the size of the convergence window for a geometry
/// optimization. It defines the number of geometry optimization steps over which
/// the energy convergence criteria must be met for convergence to be accepted.
/// # Default
/// 2
/// # Example
/// `GEOM_CONVERGENCE_WIN : 4`
pub struct GeomConvergenceWin(u32);
