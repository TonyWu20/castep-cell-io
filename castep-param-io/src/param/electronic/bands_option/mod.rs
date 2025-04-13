use castep_param_derive::{ParamDisplay, StructBuildFromPairs};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

mod extra_bands;
mod nbands;

pub use extra_bands::ExtraBands;
pub use nbands::Nbands;
/// This keyword determines the maximum number of bands at any k-point and spin.
/// There are three ways in which you can specify the maximum number of bands at any k-point and spin:
/// Directly, using `NBANDS`.
/// Indirectly, by specifying the number of extra bands in addition to the number of occupied bands using `NEXTRA_BANDS`.
/// This is the method used in the CASTEP interface.
/// Indirectly, by specifying the number of extra bands in addition to the number of occupied bands as a percentage of the latter value using `PERC_EXTRA_BANDS`.
/// It is not possible to have both the `NBANDS` keyword and either the `NEXTRA_BANDS` or `PERC_EXTRA_BANDS` keywords present in the same input file.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    Builder,
    Default,
    ParamDisplay,
    StructBuildFromPairs,
)]
#[builder(setter(into, strip_option), default)]
pub struct BandsOption {
    nbands: Option<Nbands>,
    extra_bands: Option<ExtraBands>,
}
