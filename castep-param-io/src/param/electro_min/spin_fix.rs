use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

/// This keyword determines the number of electronic iterations for which the
/// total spin is fixed. If SPIN_FIX is less than 0, the spin will be fixed for
/// the duration of the calculation.
/// This keyword only effects the behavior of the electronic minimizer in the initial SCF calculation. There is a separate keyword, GEOM_SPIN_FIX, which should be used to fix the spin of the system during geometry optimization.
/// # Note
/// This parameter is only used if FIX_OCCUPANCY : FALSE. So, for insulators the
/// spin is fixed regardless of the value of SPIN_FIX.
/// # Default
/// 10
/// # Example
/// `SPIN_FIX : 5`
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, KeywordDisplay,
)]
#[keyword_display(field = "SPIN_FIX", from=u32,value=u32,default_value=10)]
pub struct SpinFix(u32);
