use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CastepTask {
    BandStructure, // calculates band structure properties.
    GeometryOptimization, // searches for a minimum energy structure.
                   //  TODO: Future
                   // SinglePoint,            // performs a single-point energy calculation.
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

impl Display for CastepTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CastepTask::BandStructure => f.write_str("BandStructure"),
            CastepTask::GeometryOptimization => f.write_str("GeometryOptimization"),
        }
    }
}
