
use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};


#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    KeywordDisplay,
)]
#[keyword_display(field = "TASK")]
#[non_exhaustive]
pub enum CastepTask {
    BandStructure,        // calculates band structure properties.
    GeometryOptimization, // searches for a minimum energy structure.
    #[default]
    SinglePoint, // performs a single-point energy calculation.
                          //  TODO: Future
                          // MolecularDynamics,      // performs a molecular dynamics calculation.
                          // Optics,                 // calculates optical properties.
                          // Phonon, // performs a linear response calculation to determine phonon frequencies and eigenvectors.
                          // Efield, // performs an electric field linear response calculation to determine dielectric permittivity and polarizability.
                          // PhononEfield, // performs a linear response calculation to determine phonon frequencies and eigenvectors, and an electric field linear response calculation to determine dielectric permittivity and polarizability.
                          // TransitionStateSearch, // performs a transition-state search.
                          // MagRes,       // performs an NMR calculation.
                          // Elnes,        // performs core level spectroscopy calculation.
                          // ElectronicSpectroscopy, // performs electronic spectroscopy calculation.
                          // Autosolvation, // performs a free energy of solvation calculation.
}

#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::CastepTask;

    #[test]
    fn castep_task() {
        let task = CastepTask::default();
        assert_eq!("TASK : SinglePoint", task.output());
        let bandstr = CastepTask::BandStructure;
        assert_eq!("TASK : BandStructure", bandstr.output());
        let geom_opt = CastepTask::GeometryOptimization;
        assert_eq!("TASK : GeometryOptimization", geom_opt.output());
    }
}
