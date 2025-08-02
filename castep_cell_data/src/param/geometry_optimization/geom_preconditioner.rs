use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Selects the preconditioner used for LBFGS geometry optimization.
///
/// Keyword type: String
///
/// Default: GeomPreconditioner::Id
///
/// Example:
/// GEOM_PRECONDITIONER : EXP
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "GEOM_PRECONDITIONER")]
pub enum GeomPreconditioner {
    /// Identity; LBFGS is used without a preconditioner
    #[serde(alias = "id", alias = "ID")]
    Id,
    /// Exponential preconditioner
    #[serde(alias = "exp", alias = "EXP")]
    Exp,
    /// Forcefield based preconditioner using the scheme of Lindh et al. (1995)
    #[serde(alias = "ff", alias = "FF")]
    Ff,
}

impl Default for GeomPreconditioner {
    fn default() -> Self {
        Self::Id // Default is ID
    }
}

impl ToCell for GeomPreconditioner {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_PRECONDITIONER", self.to_cell_value())
    }
}

impl ToCellValue for GeomPreconditioner {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                GeomPreconditioner::Id => "ID",
                GeomPreconditioner::Exp => "EXP",
                GeomPreconditioner::Ff => "FF",
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
    fn test_geom_preconditioner_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            ("GEOM_PRECONDITIONER : Id", GeomPreconditioner::Id),
            ("GEOM_PRECONDITIONER : id", GeomPreconditioner::Id),
            ("GEOM_PRECONDITIONER : ID", GeomPreconditioner::Id), // Uppercase alias
            ("GEOM_PRECONDITIONER : Exp", GeomPreconditioner::Exp),
            ("GEOM_PRECONDITIONER : exp", GeomPreconditioner::Exp),
            ("GEOM_PRECONDITIONER : EXP", GeomPreconditioner::Exp), // Uppercase alias
            ("GEOM_PRECONDITIONER : Ff", GeomPreconditioner::Ff),
            ("GEOM_PRECONDITIONER : ff", GeomPreconditioner::Ff),
            ("GEOM_PRECONDITIONER : FF", GeomPreconditioner::Ff), // Uppercase alias
        ];

        for (input_str, expected_precon) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithGeomPreconditioner {
                geom_preconditioner: GeomPreconditioner,
            }

            let cell_file_result: Result<CellFileWithGeomPreconditioner, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.geom_preconditioner, expected_precon,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let geom_preconditioner_instance = GeomPreconditioner::Exp;
        let serialized_result = to_string(&geom_preconditioner_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized GEOM_PRECONDITIONER (EXP): {serialized_string}");
        assert!(serialized_string.contains("GEOM_PRECONDITIONER"));
        assert!(serialized_string.contains("EXP"));

        // Test Default
        assert_eq!(GeomPreconditioner::default(), GeomPreconditioner::Id);
    }
}
