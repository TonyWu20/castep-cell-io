
use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};


/// This keyword specifies the cutoff energy for the plane wave basis sets that will be used in the calculation.
/// If the BASIS_PRECISION is defined, the cutoff energy will be equal to the highest of the cutoff energies associated with the chosen level of accuracy, for the pseudopotentials used in the calculation.
/// If neither the BASIS_PRECISION nor the CUT_OFF_ENERGY are defined, the default cutoff energy is that associated with the FINE level of accuracy, for the pseudopotentials in the calculation.
/// # Note
/// It is not possible to specify both the BASIS_PRECISION and the CUT_OFF_ENERGY in a single file.
#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, PartialOrd, Default, KeywordDisplay,
)]
#[keyword_display(field="CUT_OFF_ENERGY", from=f64, value=f64, display_format="{:20.15}")]
pub struct CutoffEnergy(f64);
