use crate::param::KeywordDisplay;

use castep_param_derive::{KeywordDisplay, ParamDisplay};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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
    Builder,
    ParamDisplay,
)]
#[builder(setter(into, strip_option), default)]
pub struct CalculateProperties {
    pub stress: Option<CalculateStress>,
    pub densdiff: Option<CalculateDensdiff>,
    pub elf: Option<CalculateELF>,
    pub hirshfeld: Option<CalculateHirshfeld>,
}

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
#[keyword_display(field="CALCULATE_STRESS",from=bool,value=bool)]
pub struct CalculateStress(bool);
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
#[keyword_display(field="CALCULATE_DENSDIFF",from=bool,value=bool)]
pub struct CalculateDensdiff(bool);
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
#[keyword_display(field="CALCULATE_ELF",from=bool,value=bool)]
pub struct CalculateELF(bool);
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
#[keyword_display(field="CALCULATE_HIRSHFELD", from=bool, value=bool)]
pub struct CalculateHirshfeld(bool);

#[cfg(test)]
mod test {
    use crate::param::general::calculate_props::CalculatePropertiesBuilder;

    use super::CalculateProperties;

    #[test]
    fn calc_props() {
        let p = CalculateProperties::default();
        assert!(p.to_string().is_empty());
        let p = CalculatePropertiesBuilder::default()
            .densdiff(true)
            .stress(false)
            .build()
            .unwrap();
        let target = "CALCULATE_STRESS : false\nCALCULATE_DENSDIFF : true";
        assert_eq!(target, p.to_string());
    }
}
