use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

/// This keyword determines the number of geometry optimization steps for which the
/// total spin is fixed. If GEOM_SPIN_FIX is less than 0, the spin will be fixed to
/// the value found at the end of the SCF calculation for the initial structure for
/// the duration of the calculation.
/// This parameter is only used if FIX_OCCUPANCY = FALSE. So for insulators the spin is fixed regardless of the value of GEOM_SPIN_FIX.
///
/// The default value for this parameter is 0, so spin is allowed to vary.
/// # Example
/// `GEOM_SPIN_FIX : 5`
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    KeywordDisplay,
)]
#[keyword_display(field="GEOM_SPIN_FIX", from=u32,value=u32)]
pub struct GeomSpinFix(u32);
