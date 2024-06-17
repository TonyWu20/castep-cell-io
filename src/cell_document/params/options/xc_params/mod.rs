use std::fmt::Display;

mod xc_functional;

pub use xc_functional::XCFunctional;

use super::{OptionDisplay, ParamSectionDisplay};

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum XcParameter {
    SPIN_POLARIZED(bool),
    XC_FUNCTIONAL(XCFunctional),
}

impl OptionDisplay for XcParameter {
    fn tag(&self) -> String {
        match self {
            XcParameter::SPIN_POLARIZED(_) => "spin_polarized".to_string(),
            XcParameter::XC_FUNCTIONAL(_) => "xc_functional".to_string(),
        }
    }

    fn value(&self) -> String {
        match self {
            XcParameter::SPIN_POLARIZED(b) => format!("{b}"),
            XcParameter::XC_FUNCTIONAL(xc) => xc.value(),
        }
    }
}

impl Display for XcParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output())
    }
}

#[derive(Debug, Clone)]
pub struct XcParamSection {
    params: Vec<XcParameter>,
}

impl XcParamSection {
    pub fn new(params: Vec<XcParameter>) -> Self {
        Self { params }
    }
}

impl Default for XcParamSection {
    fn default() -> Self {
        Self::new(vec![
            XcParameter::XC_FUNCTIONAL(XCFunctional::default()),
            XcParameter::SPIN_POLARIZED(true),
        ])
    }
}

impl ParamSectionDisplay for XcParamSection {
    fn options(&self) -> &[impl Display] {
        &self.params
    }
}

impl Display for XcParamSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.section_content())
    }
}

#[cfg(test)]
mod test {
    use super::XcParamSection;

    #[test]
    fn test_xc_param() {
        let xc = XcParamSection::default();
        println!("{xc}");
    }
}
