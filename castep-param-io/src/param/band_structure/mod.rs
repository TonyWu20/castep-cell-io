use crate::param::KeywordDisplay;
use castep_param_derive::ParamDisplay;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

mod bs_eigenvalue_tol;
mod bs_extra_bands;
mod bs_max_cg_steps;
mod bs_max_iter;
mod bs_nbands;
mod bs_re_est_k_scrn;
mod bs_write_eigenvalues;
mod bs_xc_functional;

pub use bs_eigenvalue_tol::*;
pub use bs_extra_bands::BSExtraBands;
pub use bs_max_cg_steps::BSMaxCgSteps;
pub use bs_max_iter::BSMaxIter;
pub use bs_nbands::BSNbands;
pub use bs_re_est_k_scrn::BSReEstKScrn;
pub use bs_write_eigenvalues::BSWriteEigenvalues;
pub use bs_xc_functional::BSXcFunctional;

#[derive(
    Debug,
    Clone,
    Copy,
    ParamDisplay,
    PartialEq,
    PartialOrd,
    Builder,
    Default,
    Serialize,
    Deserialize,
)]
#[builder(setter(into, strip_option), default)]
pub struct BandStructure {
    pub bs_eigenvalue_tol: Option<BSEigenvalueTol>,
    pub bs_max_cg_steps: Option<BSMaxCgSteps>,
    pub bs_max_iter: Option<BSMaxIter>,
    pub bs_nbands: Option<BSNbands>,
    pub bs_extra_bands: Option<BSExtraBands>,
    pub bs_re_est_k_scrn: Option<BSReEstKScrn>,
    pub bs_xc_functional: Option<BSXcFunctional>,
    /// Possibly deprecated in versions later than 6.0?
    pub bs_write_eigenvalues: Option<BSWriteEigenvalues>,
}

#[cfg(test)]
mod test {
    use super::{BSExtraBands, BandStructureBuilder};

    #[test]
    fn band_str_param() {
        let p = BandStructureBuilder::default()
            .bs_eigenvalue_tol(1e-6)
            .bs_max_iter(6000)
            .bs_extra_bands(BSExtraBands::PercExtra(72.0))
            .build()
            .unwrap();
        println!("{p}");
    }
}
