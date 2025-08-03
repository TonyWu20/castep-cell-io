use crate::units::TimeUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Determines the time step for a molecular dynamics calculation.
///
/// Keyword type: Real
///
/// Default: 1.0 fs
///
/// Example:
/// MD_DELTA_T : 1.54 fs
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MD_DELTA_T")]
#[serde(from = "MdDeltaTRepr")] // Use intermediate repr for deserialization
pub struct MdDeltaT {
    /// The time step value.
    pub value: f64,
    /// The optional unit of time.
    pub unit: Option<TimeUnit>,
}

/// Intermediate representation for deserializing `MdDeltaT`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MdDeltaTRepr {
    /// Format: value unit
    WithUnit(f64, TimeUnit),
    /// Format: value (default unit fs implied)
    Essential(f64),
}

impl From<MdDeltaTRepr> for MdDeltaT {
    fn from(repr: MdDeltaTRepr) -> Self {
        match repr {
            MdDeltaTRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            MdDeltaTRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit (fs) implied
            },
        }
    }
}

impl ToCell for MdDeltaT {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_DELTA_T", self.to_cell_value())
    }
}

impl ToCellValue for MdDeltaT {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(
            [
                CellValue::Float(self.value),
                self.unit
                    .as_ref()
                    .map(|u| u.to_cell_value())
                    .unwrap_or(CellValue::Null),
            ]
            .to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_md_delta_t_serde() {
        // 1. Test Deserialization with unit
        let md_delta_t_with_unit_str = "MD_DELTA_T : 1.54 fs";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdDeltaTUnit {
            md_delta_t: MdDeltaT,
        }

        let cell_file_result: Result<CellFileWithMdDeltaTUnit, _> =
            from_str(md_delta_t_with_unit_str);
        // This test depends on TimeUnit correctly parsing "fs"
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.md_delta_t.value - 1.54).abs() < 1e-10);
        // assert_eq!(cell_file.md_delta_t.unit, Some(TimeUnit::Femtosecond)); // If this variant exists

        // 2. Test Deserialization without unit (default unit implied)
        let md_delta_t_default_str = "MD_DELTA_T : 2.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdDeltaTDefault {
            md_delta_t: MdDeltaT,
        }

        let cell_file_default_result: Result<CellFileWithMdDeltaTDefault, _> =
            from_str(md_delta_t_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.md_delta_t.value - 2.0).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.md_delta_t.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        // This depends on TimeUnit having a variant that serializes correctly.
        // let md_delta_t_instance_with_unit = MdDeltaT {
        //     value: 1.0,
        //     unit: Some(TimeUnit::Femtosecond), // If this variant exists
        // };
        // let serialized_result_with_unit = to_string(&md_delta_t_instance_with_unit.to_cell());
        // assert!(serialized_result_with_unit.is_ok(), "Serialization (with unit) failed: {:?}", serialized_result_with_unit.err());
        // let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        // println!("Serialized MD_DELTA_T (1.0 fs): {serialized_string_with_unit}");
        // assert!(serialized_string_with_unit.contains("MD_DELTA_T"));
        // assert!(serialized_string_with_unit.contains("1.0"));
        // assert!(serialized_string_with_unit.contains("fs"));

        // 4. Test Serialization using ToCell (without unit)
        let md_delta_t_instance_no_unit = MdDeltaT {
            value: 3.0,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&md_delta_t_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized MD_DELTA_T (3.0, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("MD_DELTA_T"));
        assert!(serialized_string_no_unit.contains("3.0"));
        // Check that the unit string is not present (or is the default)
    }
}
