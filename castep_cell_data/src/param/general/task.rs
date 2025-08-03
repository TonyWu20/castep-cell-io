use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
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

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_task_serde() {
        // Test a few key variants
        let test_cases = [
            ("TASK : SinglePoint", Task::SinglePoint),
            ("TASK : GeometryOptimization", Task::GeometryOptimization),
            ("TASK : Optics", Task::Optics),
            ("TASK : Phonon+Efield", Task::PhononPlusEfield),
        ];

        for (input_str, expected_task) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithTask {
                task: Task,
            }

            let cell_file_result: Result<CellFileWithTask, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.task, expected_task,
                "Failed for input: {input_str}"
            );
        }

        let task_instance = Task::MolecularDynamics;
        let serialized_result = to_string(&task_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized TASK (MolecularDynamics):\n{serialized_string}");
        assert!(serialized_string.contains("TASK"));
        assert!(serialized_string.contains("MolecularDynamics"));

        assert_eq!(Task::default(), Task::SinglePoint);
    }
}
