use std::fmt::Display;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

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
)]
#[builder(setter(into, strip_option), default)]
pub struct WriteProperties {
    pub orbitals: Option<bool>,
    pub formatted_elf: Option<bool>,
    pub formatted_density: Option<bool>,
    pub formatted_potential: Option<bool>,
}

impl WriteProperties {
    pub fn orbitals(&self) -> Option<bool> {
        self.orbitals
    }

    pub fn set_orbitals(&mut self, orbitals: Option<bool>) {
        self.orbitals = orbitals;
    }

    pub fn formatted_elf(&self) -> Option<bool> {
        self.formatted_elf
    }

    pub fn set_formatted_elf(&mut self, formatted_elf: Option<bool>) {
        self.formatted_elf = formatted_elf;
    }

    pub fn formatted_density(&self) -> Option<bool> {
        self.formatted_density
    }

    pub fn set_formatted_density(&mut self, formatted_density: Option<bool>) {
        self.formatted_density = formatted_density;
    }

    pub fn formatted_potential(&self) -> Option<bool> {
        self.formatted_potential
    }

    pub fn set_formatted_potential(&mut self, formatted_potential: Option<bool>) {
        self.formatted_potential = formatted_potential;
    }
}

impl Display for WriteProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = [
            self.orbitals().map(|b| format!("WRITE_ORBITALS : {}", b)),
            self.formatted_elf
                .map(|b| format!("WRITE_FORMATTED_ELF : {}", b)),
            self.formatted_density
                .map(|b| format!("WRITE_FORMATTED_DENSITY : {}", b)),
            self.formatted_potential
                .map(|b| format!("WRITE_FORMATTED_POTENTIAL : {}", b)),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join("\n");
        write!(f, "{}", output)
    }
}

impl KeywordDisplay for WriteProperties {
    fn field(&self) -> String {
        String::new()
    }
    fn output(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::WriteProperties;

    #[test]
    fn write_properties() {
        let write_prop = WriteProperties::default();
        assert_eq!("", write_prop.to_string());
        let mut write_prop = WriteProperties::default();
        write_prop.set_formatted_density(Some(true));
        write_prop.set_formatted_elf(Some(true));
        write_prop.set_formatted_potential(Some(false));
        let target = r#"WRITE_FORMATTED_ELF : true
WRITE_FORMATTED_DENSITY : true
WRITE_FORMATTED_POTENTIAL : false"#;
        assert_eq!(target, write_prop.to_string());
    }
}
