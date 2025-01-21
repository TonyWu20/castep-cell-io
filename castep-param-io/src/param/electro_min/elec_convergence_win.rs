use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field="ELEC_CONVERGENCE_WIN", from=u64,value=u64, default_value=3)]
/// This keyword determines the size of the convergence window during a electronic minimization run.
/// The total energy or eigenvalue must lie within ELEC_ENERGY_TOL or ELEC_EIGENVALUE_TOL respectively, for the last ELEC_CONVERGENCE_WIN iterations for the convergence criteria to be met.
/// # Note
/// The value of ELEC_CONVERGENCE_WIN must be greater than or equal to 2.
/// # Default
/// 3
/// # Example
/// `ELEC_CONVERGENCE_WIN : 4`
pub struct ElecConvergenceWin(u64);
