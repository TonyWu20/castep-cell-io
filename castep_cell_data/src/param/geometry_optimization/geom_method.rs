use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the method used for geometry optimization.
///
/// Keyword type: String
///
/// Default: GeomMethod::Bfgs
///
/// Example:
/// GEOM_METHOD : DampedMD
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "GEOM_METHOD")]
pub enum GeomMethod {
    /// BFGS minimization
    #[serde(alias = "bfgs", alias = "BFGS")]
    Bfgs,
    /// Low-memory BFGS minimization
    #[serde(alias = "lbfgs", alias = "LBFGS")]
    Lbfgs,
    /// BFGS minimization using delocalized internal coordinates
    #[serde(rename = "Delocalized", alias = "delocalized", alias = "DELOCALIZED")]
    #[serde(alias = "Delocalised", alias = "delocalised", alias = "DELOCALISED")]
    // Alternative spelling
    Delocalized,
    /// Damped molecular dynamics
    #[serde(alias = "dampedmd", alias = "DAMPEDMD", alias = "DampedMD")]
    DampedMd,
    /// Two-point steepest descent
    #[serde(alias = "tpsd", alias = "TPSD")]
    Tpsd,
}

impl Default for GeomMethod {
    fn default() -> Self {
        Self::Bfgs // Default is BFGS
    }
}

impl ToCell for GeomMethod {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for GeomMethod {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                GeomMethod::Bfgs => "BFGS",
                GeomMethod::Lbfgs => "LBFGS",
                GeomMethod::Delocalized => "Delocalized",
                GeomMethod::DampedMd => "DampedMD",
                GeomMethod::Tpsd => "TPSD",
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
    fn test_geom_method_serde() {
        // Test Deserialization for various cases
        let test_cases_deser = [
            ("GEOM_METHOD : BFGS", GeomMethod::Bfgs),
            ("GEOM_METHOD : bfgs", GeomMethod::Bfgs),
            ("GEOM_METHOD : BFGS", GeomMethod::Bfgs), // Uppercase alias
            ("GEOM_METHOD : LBFGS", GeomMethod::Lbfgs),
            ("GEOM_METHOD : lbfgs", GeomMethod::Lbfgs),
            ("GEOM_METHOD : LBFGS", GeomMethod::Lbfgs), // Uppercase alias
            ("GEOM_METHOD : Delocalized", GeomMethod::Delocalized),
            ("GEOM_METHOD : delocalized", GeomMethod::Delocalized),
            ("GEOM_METHOD : DELOCALIZED", GeomMethod::Delocalized), // Uppercase alias
            ("GEOM_METHOD : Delocalised", GeomMethod::Delocalized), // Alternative spelling
            ("GEOM_METHOD : delocalised", GeomMethod::Delocalized), // Alternative spelling
            ("GEOM_METHOD : DELOCALISED", GeomMethod::Delocalized), // Alternative spelling, Uppercase alias
            ("GEOM_METHOD : DampedMD", GeomMethod::DampedMd),
            ("GEOM_METHOD : dampedmd", GeomMethod::DampedMd),
            ("GEOM_METHOD : DAMPEDMD", GeomMethod::DampedMd), // Uppercase alias
            ("GEOM_METHOD : TPSD", GeomMethod::Tpsd),
            ("GEOM_METHOD : tpsd", GeomMethod::Tpsd),
            ("GEOM_METHOD : TPSD", GeomMethod::Tpsd), // Uppercase alias
        ];

        for (input_str, expected_method) in test_cases_deser {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithGeomMethod {
                geom_method: GeomMethod,
            }

            let cell_file_result: Result<CellFileWithGeomMethod, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.geom_method, expected_method,
                "Failed for input: {input_str}"
            );
        }

        // Test Serialization
        let geom_method_instance = GeomMethod::DampedMd;
        let serialized_result = to_string(&geom_method_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized GEOM_METHOD (DampedMD): {serialized_string}");
        assert!(serialized_string.contains("GEOM_METHOD"));
        assert!(serialized_string.contains("DampedMD"));

        // Test Default
        assert_eq!(GeomMethod::default(), GeomMethod::Bfgs);
    }
}
