use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Defines the type of calculation to perform.
///
/// Keyword type: String
///
/// Default: Task::SinglePoint
///
/// Example:
/// TASK : optics
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "TASK")]
pub enum Task {
    /// Performs a single-point energy calculation.
    #[serde(alias = "SINGLEPOINT", alias = "singlepoint")]
    SinglePoint,
    /// Calculates band structure properties.
    #[serde(alias = "BANDSTRUCTURE", alias = "bandstructure")]
    BandStructure,
    /// Searches for a minimum energy structure.
    #[serde(alias = "GEOMETRYOPTIMIZATION", alias = "geometryoptimization")]
    GeometryOptimization,
    /// Performs a molecular dynamics calculation.
    #[serde(alias = "MOLECULARDYNAMICS", alias = "moleculardynamics")]
    MolecularDynamics,
    /// Calculates optical properties.
    #[serde(alias = "OPTICS", alias = "optics")]
    Optics,
    /// Performs a linear response calculation to determine phonon frequencies and eigenvectors.
    #[serde(alias = "PHONON", alias = "phonon")]
    Phonon,
    /// Performs an electric field linear response calculation.
    #[serde(alias = "EFIELD", alias = "efield")]
    Efield,
    /// Performs both Phonon and Efield calculations.
    #[serde(alias = "PHONON+EFIELD", alias = "phonon+efield")]
    PhononPlusEfield,
    /// Performs a transition-state search.
    #[serde(alias = "TRANSITIONSTATESEARCH", alias = "transitionstatesearch")]
    TransitionStateSearch,
    /// Performs an NMR calculation.
    #[serde(alias = "MAGRES", alias = "magres")]
    MagRes,
    /// Performs core level spectroscopy calculation.
    #[serde(alias = "ELNES", alias = "elnes")]
    Elnes,
    /// Performs electronic spectroscopy calculation.
    #[serde(alias = "ELECTRONICSPECTROSCOPY", alias = "electronicspectroscopy")]
    ElectronicSpectroscopy,
    /// Performs a free energy of solvation calculation.
    #[serde(alias = "AUTOSOLVATION", alias = "autosolvation")]
    Autosolvation,
}

impl Default for Task {
    fn default() -> Self {
        Self::SinglePoint // Default is SinglePoint
    }
}

impl FromCellValue for Task {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "singlepoint" => Ok(Self::SinglePoint),
            "bandstructure" => Ok(Self::BandStructure),
            "geometryoptimization" => Ok(Self::GeometryOptimization),
            "moleculardynamics" => Ok(Self::MolecularDynamics),
            "optics" => Ok(Self::Optics),
            "phonon" => Ok(Self::Phonon),
            "efield" => Ok(Self::Efield),
            "phonon+efield" => Ok(Self::PhononPlusEfield),
            "transitionstatesearch" => Ok(Self::TransitionStateSearch),
            "magres" => Ok(Self::MagRes),
            "elnes" => Ok(Self::Elnes),
            "electronicspectroscopy" => Ok(Self::ElectronicSpectroscopy),
            "autosolvation" => Ok(Self::Autosolvation),
            other => Err(Error::Message(format!("unknown Task: {other}"))),
        }
    }
}

impl FromKeyValue for Task {
    const KEY_NAME: &'static str = "TASK";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for Task {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("TASK", self.to_cell_value())
    }
}

impl ToCellValue for Task {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                Task::SinglePoint => "SinglePoint",
                Task::BandStructure => "BandStructure",
                Task::GeometryOptimization => "GeometryOptimization",
                Task::MolecularDynamics => "MolecularDynamics",
                Task::Optics => "Optics",
                Task::Phonon => "Phonon",
                Task::Efield => "Efield",
                Task::PhononPlusEfield => "Phonon+Efield",
                Task::TransitionStateSearch => "TransitionStateSearch",
                Task::MagRes => "MagRes",
                Task::Elnes => "Elnes",
                Task::ElectronicSpectroscopy => "ElectronicSpectroscopy",
                Task::Autosolvation => "Autosolvation",
            }
            .to_string(),
        )
    }
}


