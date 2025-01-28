use castep_param_derive::{BuildFromPairs, KeywordDisplay, ParamDisplay, StructBuildFromPairs};
use derive_builder::Builder;
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::{
    param::KeywordDisplay,
    parser::{data_type::Logical, Rule},
};

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
pub struct WriteProperties {
    pub orbitals: Option<WriteOrbitals>,
    pub formatted_elf: Option<WriteFormattedELF>,
    pub formatted_density: Option<WriteFormattedDensity>,
    pub formatted_potential: Option<WriteFormattedPotential>,
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
    BuildFromPairs,
    FromPest,
)]
#[keyword_display(field = "WRITE_ORBITALS")]
#[pest_ast(rule(Rule::write_orbitals))]
#[pest_rule(rule=Rule::write_orbitals, keyword="WRITE_ORBITALS")]
pub enum WriteOrbitals {
    #[pest_ast(inner(
        rule(Rule::logical),
        with(Logical::from),
        with(bool::from),
        with(WriteOrbitals::from)
    ))]
    True,
    #[default]
    False,
}

impl From<bool> for WriteOrbitals {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
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
#[keyword_display(field = "WRITE_FORMATTED_ELF")]
#[pest_ast(rule(Rule::write_formatted_elf))]
#[pest_rule(rule=Rule::write_formatted_elf, keyword="WRITE_FORMATTED_ELF")]
pub enum WriteFormattedELF {
    #[pest_ast(inner(
        rule(Rule::logical),
        with(Logical::from),
        with(bool::from),
        with(WriteFormattedELF::from)
    ))]
    True,
    #[default]
    False,
}

impl From<bool> for WriteFormattedELF {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
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
#[keyword_display(field = "WRITE_FORMATTED_DENSITY")]
#[pest_ast(rule(Rule::write_formatted_density))]
#[pest_rule(rule=Rule::write_formatted_density, keyword="WRITE_FORMATTED_DENSITY")]
pub enum WriteFormattedDensity {
    #[pest_ast(inner(
        rule(Rule::logical),
        with(Logical::from),
        with(bool::from),
        with(WriteFormattedDensity::from)
    ))]
    True,
    #[default]
    False,
}

impl From<bool> for WriteFormattedDensity {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
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
#[keyword_display(field = "WRITE_FORMATTED_POTENTIAL")]
#[pest_ast(rule(Rule::write_formatted_potential))]
#[pest_rule(rule=Rule::write_formatted_potential, keyword="WRITE_FORMATTED_POTENTIAL")]
pub enum WriteFormattedPotential {
    #[pest_ast(inner(
        rule(Rule::logical),
        with(Logical::from),
        with(bool::from),
        with(WriteFormattedPotential::from)
    ))]
    True,
    #[default]
    False,
}

impl From<bool> for WriteFormattedPotential {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::param::{WriteFormattedDensity, WriteFormattedELF, WriteFormattedPotential};

    use super::WriteProperties;

    #[test]
    fn write_properties() {
        let write_prop = WriteProperties::default();
        assert_eq!("", write_prop.to_string());
        let mut write_prop = WriteProperties::default();
        write_prop.formatted_density = Some(WriteFormattedDensity::True);
        write_prop.formatted_elf = Some(WriteFormattedELF::True);
        write_prop.formatted_potential = Some(WriteFormattedPotential::False);
        let target = r#"WRITE_FORMATTED_ELF : true
WRITE_FORMATTED_DENSITY : true
WRITE_FORMATTED_POTENTIAL : false"#;
        assert_eq!(target, write_prop.to_string());
    }
}
