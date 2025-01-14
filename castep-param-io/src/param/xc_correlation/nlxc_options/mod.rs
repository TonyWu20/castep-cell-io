use std::fmt::Display;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

pub use exchange_reflect_kpts::ExchangeReflectKpts;
pub use impose_trs::ImposeTrs;
pub use k_scrn_averaging_scheme::KScrnAveragingScheme;
pub use page_ex_pot::PageExPot;
pub use ppd_integral::PPDIntegral;
pub use ppd_size::{PPDSizeX, PPDSizeY, PPDSizeZ};
pub use re_est_k_scrn::ReEstKScrn;

mod exchange_reflect_kpts;
mod impose_trs;
mod k_scrn_averaging_scheme;
mod page_ex_pot;
mod ppd_integral;
mod ppd_size;
mod re_est_k_scrn;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default, Builder,
)]
#[builder(setter(into, strip_option), default)]
pub struct NLXCOptions {
    impose_trs: Option<ImposeTrs>,
    k_scan_averaging_scheme: Option<KScrnAveragingScheme>,
    re_est_k_scrn: Option<ReEstKScrn>,
    exchange_reflect_kpts: Option<ExchangeReflectKpts>,
    page_ex_pot: Option<PageExPot>,
    /// This keyword enables you to use parallelepiped integration to effectively smear out each k-point used in the SCF calculation onto a parallelepiped whose dimensions are determined by the spacing of the Monkhorst-Pack grid. This technique is useful for band structure calculations with exact or screened exchange functionals if the band structure path passes very close to one of the SCF k-points. The NLXC_PPD_INTEGRAL is the latest form of this keyword, while the obsolete version, PPD_INTEGRAL, is supported for backward compatibility.
    ppd_integral: Option<PPDIntegral>,
    ppd_size_x: Option<PPDSizeX>,
    ppd_size_y: Option<PPDSizeY>,
    ppd_size_z: Option<PPDSizeZ>,
}

impl Display for NLXCOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = [
            self.impose_trs.map(|v| v.output()),
            self.k_scan_averaging_scheme.map(|v| v.output()),
            self.re_est_k_scrn.map(|v| v.output()),
            self.exchange_reflect_kpts.map(|v| v.output()),
            self.page_ex_pot.map(|v| v.output()),
            self.ppd_integral.map(|v| v.output()),
            self.ppd_size_x.map(|v| v.output()),
            self.ppd_size_y.map(|v| v.output()),
            self.ppd_size_z.map(|v| v.output()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join("\n");
        write!(f, "{}", output)
    }
}
