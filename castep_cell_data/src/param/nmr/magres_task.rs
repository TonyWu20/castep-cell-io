use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Defines the type of NMR calculation to be performed.
///
/// Keyword type: String
///
/// Default: MagresTask::Shielding
///
/// Example:
/// MAGRES_TASK : NMR
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MAGRES_TASK")]
pub enum MagresTask {
    /// Performs a calculation of the NMR shielding tensor for all atoms
    #[serde(alias = "shielding", alias = "SHIELDING")]
    Shielding,
    /// Performs a calculation of the electric field gradient tensor for all atoms
    #[serde(alias = "efg", alias = "EFG")]
    Efg,
    /// Performs a calculation of both the NMR shielding tensor and the EFG tensor
    #[serde(alias = "nmr", alias = "NMR")]
    Nmr,
}

impl Default for MagresTask {
    fn default() -> Self {
        Self::Shielding // Default is Shielding
    }
}

impl ToCell for MagresTask {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MAGRES_TASK", self.to_cell_value())
    }
}

impl ToCellValue for MagresTask {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MagresTask::Shielding => "Shielding",
                MagresTask::Efg => "EFG",
                MagresTask::Nmr => "NMR",
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
    fn test_magres_task_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            ("MAGRES_TASK : Shielding", MagresTask::Shielding),
            ("MAGRES_TASK : shielding", MagresTask::Shielding),
            ("MAGRES_TASK : SHIELDING", MagresTask::Shielding), // Uppercase alias
            ("MAGRES_TASK : EFG", MagresTask::Efg),
            ("MAGRES_TASK : efg", MagresTask::Efg),
            ("MAGRES_TASK : EFG", MagresTask::Efg), // Uppercase alias
            ("MAGRES_TASK : NMR", MagresTask::Nmr),
            ("MAGRES_TASK : nmr", MagresTask::Nmr),
            ("MAGRES_TASK : NMR", MagresTask::Nmr), // Uppercase alias
        ];

        for (input_str, expected_task) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithMagresTask {
                magres_task: MagresTask,
            }

            let cell_file_result: Result<CellFileWithMagresTask, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.magres_task, expected_task,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let magres_task_instance = MagresTask::Nmr;
        let serialized_result = to_string(&magres_task_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MAGRES_TASK (NMR): {serialized_string}");
        assert!(serialized_string.contains("MAGRES_TASK"));
        assert!(serialized_string.contains("NMR"));

        // Test Default
        assert_eq!(MagresTask::default(), MagresTask::Shielding);
    }
}
