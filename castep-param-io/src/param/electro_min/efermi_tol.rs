use std::fmt::Display;

use castep_param_derive::KeywordDisplay;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::EnergyUnit;

use super::elec_eigenvalue_tol::ElecEigenvalueTol;

#[derive(
    Debug,
    Clone,
    Copy,
    Deserialize,
    Serialize,
    PartialEq,
    PartialOrd,
    Default,
    Builder,
    KeywordDisplay,
)]
#[builder(setter(into), default)]
#[keyword_display(field = "EFERMI_TOL", direct_display = false)]
/// This keyword controls the tolerance for accepting convergence of the
/// Fermi-energy if the system is being treated as a metal.
/// #Note
/// This parameter is used only if FIX_OCCUPANCY : FALSE.
/// # Default
/// `0.1 × ELEC_EIGENVALUE_TOL`
/// # Example
/// `EFERMI_TOL : 0.0000007 eV`
pub struct EFermiTol {
    pub tol: f64,
    pub unit: Option<EnergyUnit>,
}

impl EFermiTol {
    pub fn default_value(elec_eigenvalue_tol: ElecEigenvalueTol) -> Self {
        Self {
            tol: 0.1 * elec_eigenvalue_tol.tol,
            unit: elec_eigenvalue_tol.unit,
        }
    }
}

impl Display for EFermiTol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:20.15} {}",
            self.tol,
            self.unit.map(|v| v.to_string()).unwrap_or_default()
        )
    }
}
