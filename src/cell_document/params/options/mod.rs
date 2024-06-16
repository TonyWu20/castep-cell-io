mod band_structure_params;
mod basis_set;
mod electronic_minimization;
mod electronic_params;
mod general;
mod geom_params;
mod populations;
mod xc_params;
// TODO: Future
mod pseudopotentials;

use std::fmt::Display;

pub use basis_set::BasisSetParamSection;
pub use electronic_minimization::ElectroMinParamSection;
pub use general::GeneralParamSection;
pub use geom_params::GeomOptParamSection;
pub use xc_params::{XCFunctional, XcParamSection};

pub trait OptionDisplay {
    fn tag(&self) -> String;
    fn value(&self) -> String;
    fn output(&self) -> String {
        format!("{} : {}", self.tag(), self.value())
    }
}

pub trait SectionDisplay {
    fn options(&self) -> &[impl Display];
    /// Join lines of `Option`
    fn section_content(&self) -> String {
        self.options()
            .iter()
            .map(|item| format!("{item}"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
