use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

/// This keyword selects the preconditioner used for LBFGS geometry optimization.
/// Available options are:
/// - ID - identity; LBFGS is used without a preconditioner.
/// - EXP - exponential preconditioner.
/// - FF- forcefield based preconditioner using the scheme of Lindh et al. (1995).
///
/// The ID option's only advantage over the BFGS minimizer is lower memory requirements.
/// The EXP option is generally the best in terms of performance gains.
/// The forcefield based preconditioner FF can sometimes provide further gains,
/// although it is less stable and might require more steps to converge.
/// # Default
/// ID
/// # Example
/// `GEOM_PRECONDITIONER : EXP`
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Hash,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    KeywordDisplay,
)]
#[keyword_display(field = "GEOM_PRECONDITIONER")]
#[allow(clippy::upper_case_acronyms)]
pub enum GeomPreconditioner {
    #[default]
    ID,
    EXP,
    FF,
}
