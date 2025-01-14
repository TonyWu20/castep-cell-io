use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

/// This keyword determines which mixing scheme will be used in the density mixing
/// procedure.
/// Available options are:
/// - Kerker
/// - Linear
/// - Broyden
/// - Pulay
/// # Default
/// `Broyden`
/// # Example
/// `MIXING_SCHEME : Pulay`
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Default,
    KeywordDisplay,
)]
#[keyword_display(field = "MIXING_SCHEME")]
pub enum MixingScheme {
    Kerker,
    Linear,
    #[default]
    Broyden,
    Pulay,
}
