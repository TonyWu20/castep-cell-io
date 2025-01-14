use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::{EnergyUnit, Nbands};

use super::ElecEnergyTol;

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, PartialOrd, Builder, KeywordDisplayStruct,
)]
#[builder(setter(into), default)]
#[keyword_display(field = "ELEC_EIGENVALUE_TOL", display_format = "{:20.15e} {}", from=f64, default_value=1e-6)]
/// This keyword controls the tolerance for accepting convergence of a single
/// eigenvalue during density mixing minimization.
/// The difference between maximum and minimum eigenvalues over ELEC_CONVERGENCE_WIN
/// iterations must be less than this value.
/// # Default
/// The default value is the lower of 1x10-6 eV and ELEC_ENERGY_TOL*NATOMS/NBANDS, where NATOMS is the total number of atoms in the unit cell.
pub struct ElecEigenvalueTol {
    pub tol: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<EnergyUnit>,
}

impl ElecEigenvalueTol {
    fn default_value(elec_energy_tol: ElecEnergyTol, natoms: usize, nbands: Nbands) -> Self {
        let tol = elec_energy_tol.tol * natoms as f64 / nbands.value() as f64;
        Self {
            tol,
            unit: elec_energy_tol.unit,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::ElecEigenvalueTol;

    #[test]
    fn elec_eigenvalue_tol() {
        let p = ElecEigenvalueTol::default();
        println!("{}", p.output())
    }
}
