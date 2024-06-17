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

pub use band_structure_params::*;
pub use basis_set::*;
pub use electronic_minimization::*;
pub use electronic_params::*;
pub use general::*;
pub use geom_params::*;
pub use populations::*;
pub use xc_params::*;

pub trait OptionDisplay {
    fn tag(&self) -> String;
    fn value(&self) -> String;
    fn output(&self) -> String {
        format!("{} : {}", self.tag(), self.value())
    }
}

pub trait ParamSectionDisplay {
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
