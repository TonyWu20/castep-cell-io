use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the ensemble used for a molecular dynamics calculation.
///
/// Keyword type: String
///
/// Default: MdEnsemble::Nve
///
/// Example:
/// MD_ENSEMBLE : NVT
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MD_ENSEMBLE")]
pub enum MdEnsemble {
    /// Canonical ensemble (constant number of particles, volume, temperature)
    #[serde(alias = "nvt", alias = "NVT")]
    Nvt,
    /// Microcanonical ensemble (constant number of particles, volume, energy)
    #[serde(alias = "nve", alias = "NVE")]
    Nve,
    /// Isothermal-isobaric ensemble (constant number of particles, pressure, temperature)
    #[serde(alias = "npt", alias = "NPT")]
    Npt,
    /// Isenthalpic-isobaric ensemble (constant number of particles, pressure, enthalpy)
    #[serde(alias = "nph", alias = "NPH")]
    Nph,
}

impl Default for MdEnsemble {
    fn default() -> Self {
        Self::Nve // Default is NVE
    }
}

impl ToCell for MdEnsemble {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_ENSEMBLE", self.to_cell_value())
    }
}

impl ToCellValue for MdEnsemble {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MdEnsemble::Nvt => "NVT",
                MdEnsemble::Nve => "NVE",
                MdEnsemble::Npt => "NPT",
                MdEnsemble::Nph => "NPH",
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
    fn test_md_ensemble_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            ("MD_ENSEMBLE : NVT", MdEnsemble::Nvt),
            ("MD_ENSEMBLE : nvt", MdEnsemble::Nvt),
            ("MD_ENSEMBLE : NVT", MdEnsemble::Nvt), // Uppercase alias
            ("MD_ENSEMBLE : NVE", MdEnsemble::Nve),
            ("MD_ENSEMBLE : nve", MdEnsemble::Nve),
            ("MD_ENSEMBLE : NVE", MdEnsemble::Nve), // Uppercase alias
            ("MD_ENSEMBLE : NPT", MdEnsemble::Npt),
            ("MD_ENSEMBLE : npt", MdEnsemble::Npt),
            ("MD_ENSEMBLE : NPT", MdEnsemble::Npt), // Uppercase alias
            ("MD_ENSEMBLE : NPH", MdEnsemble::Nph),
            ("MD_ENSEMBLE : nph", MdEnsemble::Nph),
            ("MD_ENSEMBLE : NPH", MdEnsemble::Nph), // Uppercase alias
        ];

        for (input_str, expected_ensemble) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithMdEnsemble {
                md_ensemble: MdEnsemble,
            }

            let cell_file_result: Result<CellFileWithMdEnsemble, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.md_ensemble, expected_ensemble,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let md_ensemble_instance = MdEnsemble::Nvt;
        let serialized_result = to_string(&md_ensemble_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized MD_ENSEMBLE (NVT): {serialized_string}");
        assert!(serialized_string.contains("MD_ENSEMBLE"));
        assert!(serialized_string.contains("NVT"));

        // Test Default
        assert_eq!(MdEnsemble::default(), MdEnsemble::Nve);
    }
}
