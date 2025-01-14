use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::param::KeywordDisplay;

use derive_builder::Builder;
pub use pspot_beta_phi_type::PSPotBetaPhiType;
pub use pspot_nonlocal_type::PSPotNonlocalType;
pub use relativistic_treatment::RelativisticTreatment;

mod pspot_beta_phi_type;
mod pspot_nonlocal_type;
mod relativistic_treatment;

#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Builder,
    Default,
)]
#[builder(
    setter(into, strip_option),
    default,
    build_fn(validate = "Self::validate")
)]
pub struct Pseudopotentials {
    pub pspot_beta_phi_type: Option<PSPotBetaPhiType>,
    pub pspot_nonlocal_type: Option<PSPotNonlocalType>,
    pub relativistic_treatment: Option<RelativisticTreatment>,
}

impl Display for Pseudopotentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = [
            self.pspot_beta_phi_type.map(|v| v.output()),
            self.pspot_nonlocal_type.map(|v| v.output()),
            self.relativistic_treatment.map(|v| v.output()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join("\n");
        write!(f, "{}", output)
    }
}
impl PseudopotentialsBuilder {
    fn validate(builder: &PseudopotentialsBuilder) -> Result<(), String> {
        if let Some(Some(non)) = builder.pspot_nonlocal_type {
            match non {
            PSPotNonlocalType::Reciprocal => match builder.pspot_beta_phi_type {
                Some(Some(b)) => match b {
                            PSPotBetaPhiType::Reciprocal => Ok(()),
                            PSPotBetaPhiType::Real => Err("The `pspot_beta_phi_type` can only take `REAL` if `pspot_nonlocal_type` is also `REAL`".to_string())
                        },
                Some(None) => Ok(()),
                None => Ok(()),
                },
            PSPotNonlocalType::Real => Ok(()),
        }
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::param::pseudopotentials::{
        PSPotBetaPhiType, PSPotNonlocalType, RelativisticTreatment,
    };

    use super::PseudopotentialsBuilder;

    #[test]
    fn pseudopotentials() {
        let p = PseudopotentialsBuilder::default()
            .pspot_beta_phi_type(PSPotBetaPhiType::Real)
            .pspot_nonlocal_type(PSPotNonlocalType::Real)
            .relativistic_treatment(RelativisticTreatment::Zora)
            .build()
            .unwrap();
        println!("{p}");
        let p = PseudopotentialsBuilder::default()
            .pspot_beta_phi_type(PSPotBetaPhiType::Real)
            .pspot_nonlocal_type(PSPotNonlocalType::Reciprocal)
            .relativistic_treatment(RelativisticTreatment::Zora)
            .build();
        assert!(p.is_err());
        let p = PseudopotentialsBuilder::default()
            .pspot_nonlocal_type(PSPotNonlocalType::Reciprocal)
            .relativistic_treatment(RelativisticTreatment::Zora)
            .build();
        assert!(p.is_ok());
        let p = PseudopotentialsBuilder::default()
            .pspot_beta_phi_type(PSPotBetaPhiType::Reciprocal)
            .pspot_nonlocal_type(PSPotNonlocalType::Real)
            .relativistic_treatment(RelativisticTreatment::Zora)
            .build();
        assert!(p.is_ok());
    }
}
