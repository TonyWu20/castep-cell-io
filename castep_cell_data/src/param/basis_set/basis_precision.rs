use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Specifies the precision of the basis set by choosing the level of convergence
/// of atomic energies with respect to the plane wave cutoff energy.
///
/// Keyword type: String
///
/// Default: BasisPrecision::Fine
///
/// Example:
/// BASIS_PRECISION : MEDIUM
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BASIS_PRECISION")]
pub enum BasisPrecision {
    /// Convergence of about 1 eV/atom
    #[serde(alias = "coarse", alias = "COARSE")]
    Coarse,
    /// Convergence of about 0.3 eV/atom
    #[serde(alias = "medium", alias = "MEDIUM")]
    Medium,
    /// Convergence of about 0.1 eV/atom
    #[serde(alias = "fine", alias = "FINE")]
    Fine,
    /// 1.2 × FINE cutoff energy
    #[serde(alias = "precise", alias = "PRECISE")]
    Precise,
    /// 1.6 × FINE cutoff energy, convergence of about 0.01 eV/atom
    #[serde(alias = "extreme", alias = "EXTREME")]
    Extreme,
}

impl Default for BasisPrecision {
    fn default() -> Self {
        Self::Fine // Default is FINE
    }
}

impl ToCell for BasisPrecision {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BASIS_PRECISION", self.to_cell_value())
    }
}

impl ToCellValue for BasisPrecision {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                BasisPrecision::Coarse => "COARSE",
                BasisPrecision::Medium => "MEDIUM",
                BasisPrecision::Fine => "FINE",
                BasisPrecision::Precise => "PRECISE",
                BasisPrecision::Extreme => "EXTREME",
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
    fn test_basis_precision_serde() {
        let test_cases = [
            ("BASIS_PRECISION : COARSE", BasisPrecision::Coarse),
            ("BASIS_PRECISION : MEDIUM", BasisPrecision::Medium),
            ("BASIS_PRECISION : FINE", BasisPrecision::Fine),
            ("BASIS_PRECISION : PRECISE", BasisPrecision::Precise),
            ("BASIS_PRECISION : EXTREME", BasisPrecision::Extreme),
        ];

        for (input_str, expected_precision) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithBasisPrecision {
                basis_precision: BasisPrecision,
            }

            let cell_file_result: Result<CellFileWithBasisPrecision, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.basis_precision, expected_precision,
                "Failed for input: {input_str}"
            );
        }

        let basis_precision_instance = BasisPrecision::Medium;
        let serialized_result = to_string(&basis_precision_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized BASIS_PRECISION (MEDIUM): {serialized_string}");
        assert!(serialized_string.contains("BASIS_PRECISION"));
        assert!(serialized_string.contains("MEDIUM"));

        assert_eq!(BasisPrecision::default(), BasisPrecision::Fine);
    }
}
