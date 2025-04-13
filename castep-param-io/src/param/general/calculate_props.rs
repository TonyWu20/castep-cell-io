use crate::{
    param::KeywordDisplay,
    parser::{data_type::Logical, Rule},
};
use from_pest::FromPest;
use pest::Parser;

use castep_param_derive::{BuildFromPairs, KeywordDisplay, ParamDisplay, StructBuildFromPairs};
use derive_builder::Builder;
use pest_ast::FromPest;
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
    StructBuildFromPairs,
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
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="CALCULATE_STRESS",from=bool,value=bool)]
#[pest_ast(rule(Rule::calc_stress))]
#[pest_rule(rule=Rule::calc_stress, keyword="CALCULATE_STRESS")]
pub struct CalculateStress(
    #[pest_ast(inner(rule(Rule::logical), with(Logical::from), with(Logical::into)))] bool,
);
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
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="CALCULATE_DENSDIFF",from=bool,value=bool)]
#[pest_ast(rule(Rule::calc_densdiff))]
#[pest_rule(rule=Rule::calc_densdiff, keyword="CALCULATE_DENSDIFF")]
pub struct CalculateDensdiff(
    #[pest_ast(inner(rule(Rule::logical), with(Logical::from), with(Logical::into)))] bool,
);

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
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="CALCULATE_ELF",from=bool,value=bool)]
#[pest_ast(rule(Rule::calc_elf))]
#[pest_rule(rule=Rule::calc_elf, keyword="CALCULATE_ELF")]
pub struct CalculateELF(
    #[pest_ast(inner(rule(Rule::logical), with(Logical::from), with(Logical::into)))] bool,
);

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
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="CALCULATE_HIRSHFELD", from=bool, value=bool)]
#[pest_ast(rule(Rule::calc_hirshfeld))]
#[pest_rule(rule=Rule::calc_hirshfeld, keyword="CALCULATE_HIRSHFELD")]
pub struct CalculateHirshfeld(
    #[pest_ast(inner(rule(Rule::logical), with(Logical::from), with(Logical::into)))] bool,
);

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{
        param::general::calculate_props::CalculatePropertiesBuilder,
        parser::{ParamFile, ParamParser, Rule},
    };

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
    #[test]
    fn calc_props_parsing() {
        let input = [
            "calculate_stress : true",
            "calculate_ELF : true",
            "calculate_densdiff : True",
            "calculate_hirshfeld : false",
        ];
        input.iter().for_each(|&l| {
            let parse = ParamParser::parse(Rule::kv_pair, l).is_ok();
            assert!(parse);
        });
        let all = input.join("\n");
        let mut parse = ParamParser::parse(Rule::param_file, &all).unwrap();
        dbg!(&parse);
        let file = ParamFile::from_pest(&mut parse);
        dbg!(file.is_ok());
    }
}
