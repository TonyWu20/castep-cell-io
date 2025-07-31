use castep_cell_serde::{Cell, CellValue, Error, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines whether or not to apply a finite basis set correction
/// to energy and stress when cell parameters change.
///
/// Keyword type: Integer
///
/// Default: 2
///
/// Example:
/// FINITE_BASIS_CORR : 1
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "FINITE_BASIS_CORR")]
#[serde(try_from = "FiniteBasisCorrRepr")]
pub enum FiniteBasisCorr {
    /// No correction.
    #[serde(rename = "0")]
    None,
    /// Manual correction using specified BASIS_DE_DLOGE.
    #[serde(rename = "1")]
    Manual,
    /// Automatic correction using FINITE_BASIS_NPOINTS and FINITE_BASIS_SPACING.
    #[serde(rename = "2")]
    Automatic,
}
#[derive(Debug, Deserialize)]
struct FiniteBasisCorrRepr(u32);

impl TryFrom<FiniteBasisCorrRepr> for FiniteBasisCorr {
    type Error = Error;

    fn try_from(value: FiniteBasisCorrRepr) -> Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(Self::None),
            1 => Ok(Self::Manual),
            2 => Ok(Self::Automatic),
            _ => Err(Error::Message(
                "value of `FiniteBasisCorr` exceeds maximum of 2".to_string(),
            )),
        }
    }
}

impl Default for FiniteBasisCorr {
    fn default() -> Self {
        Self::Automatic // Default is 2
    }
}

impl ToCell for FiniteBasisCorr {
    fn to_cell(&self) -> Cell {
        let value = match self {
            FiniteBasisCorr::None => 0,
            FiniteBasisCorr::Manual => 1,
            FiniteBasisCorr::Automatic => 2,
        };
        Cell::KeyValue("FINITE_BASIS_CORR", CellValue::Int(value))
    }
}

impl ToCellValue for FiniteBasisCorr {
    fn to_cell_value(&self) -> CellValue {
        let value = match self {
            FiniteBasisCorr::None => 0,
            FiniteBasisCorr::Manual => 1,
            FiniteBasisCorr::Automatic => 2,
        };
        CellValue::Int(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_finite_basis_corr_serde() {
        let test_cases = [
            ("FINITE_BASIS_CORR : 0", FiniteBasisCorr::None),
            ("FINITE_BASIS_CORR : 1", FiniteBasisCorr::Manual),
            ("FINITE_BASIS_CORR : 2", FiniteBasisCorr::Automatic),
        ];

        for (input_str, expected_corr) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithFiniteBasisCorr {
                finite_basis_corr: FiniteBasisCorr,
            }

            let cell_file_result: Result<CellFileWithFiniteBasisCorr, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.finite_basis_corr, expected_corr,
                "Failed for input: {input_str}"
            );
        }

        let finite_basis_corr_instance = FiniteBasisCorr::Manual;
        let serialized_result = to_string(&finite_basis_corr_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized FINITE_BASIS_CORR (1): {serialized_string}");
        assert!(serialized_string.contains("FINITE_BASIS_CORR"));
        assert!(serialized_string.contains("1"));

        assert_eq!(FiniteBasisCorr::default(), FiniteBasisCorr::Automatic);
    }
}
