use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls the percentage of extra bands at each k-point in addition to the number of occupied bands.
///
/// Keyword type: Real
///
/// Default: 0.0
///
/// Example:
/// BS_PERC_EXTRA_BANDS : 60.0
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "BS_PERC_EXTRA_BANDS")]
pub struct BsPercExtraBands(pub f64);

impl ToCell for BsPercExtraBands {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BS_PERC_EXTRA_BANDS", CellValue::Float(self.0))
    }
}

impl ToCellValue for BsPercExtraBands {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Float(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_bs_perc_extra_bands_serde() {
        let bs_perc_extra_bands_str = "BS_PERC_EXTRA_BANDS : 60.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithBsPercExtraBands {
            bs_perc_extra_bands: BsPercExtraBands,
        }

        let cell_file_result: Result<CellFileWithBsPercExtraBands, _> =
            from_str(bs_perc_extra_bands_str);
        assert!(
            cell_file_result.is_ok(),
            "Deserialization failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.bs_perc_extra_bands.0 - 60.0).abs() < f64::EPSILON);

        let bs_perc_extra_bands_instance = BsPercExtraBands(50.5);
        let serialized_result = to_string(&bs_perc_extra_bands_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized BS_PERC_EXTRA_BANDS (50.5): {serialized_string}");
        assert!(serialized_string.contains("BS_PERC_EXTRA_BANDS"));
        assert!(serialized_string.contains("50.5"));

        assert_eq!(BsPercExtraBands::default(), BsPercExtraBands(0.0));
    }
}
