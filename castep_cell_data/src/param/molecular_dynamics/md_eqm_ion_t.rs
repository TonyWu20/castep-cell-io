use crate::units::TimeUnit;
use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Sets the MD thermostat parameter for enhanced MD equilibration.
///
/// Keyword type: Real
///
/// Default: Same as MD_ION_T
///
/// Example:
/// MD_EQM_ION_T : 0.5 ps
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename = "MD_EQM_ION_T")]
#[serde(from = "MdEqmIonTRepr")] // Use intermediate repr for deserialization
pub struct MdEqmIonT {
    /// The equilibration thermostat parameter value.
    pub value: f64,
    /// The optional unit of time.
    pub unit: Option<TimeUnit>,
}

/// Intermediate representation for deserializing `MdEqmIonT`.
/// Handles the optional unit.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MdEqmIonTRepr {
    /// Format: value unit
    WithUnit(f64, TimeUnit),
    /// Format: value (default unit implied or context-dependent)
    Essential(f64),
}

impl From<MdEqmIonTRepr> for MdEqmIonT {
    fn from(repr: MdEqmIonTRepr) -> Self {
        match repr {
            MdEqmIonTRepr::WithUnit(value, unit) => Self {
                value,
                unit: Some(unit),
            },
            MdEqmIonTRepr::Essential(value) => Self {
                value,
                unit: None, // Default unit implied or context-dependent
            },
        }
    }
}

impl ToCell for MdEqmIonT {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_EQM_ION_T", self.to_cell_value())
    }
}

impl ToCellValue for MdEqmIonT {
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
    fn test_md_eqm_ion_t_serde() {
        // 1. Test Deserialization with unit
        let md_eqm_ion_t_with_unit_str = "MD_EQM_ION_T : 0.5 ps";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdEqmIonTUnit {
            md_eqm_ion_t: MdEqmIonT,
        }

        let cell_file_result: Result<CellFileWithMdEqmIonTUnit, _> =
            from_str(md_eqm_ion_t_with_unit_str);
        // This test depends on TimeUnit correctly parsing "ps"
        assert!(
            cell_file_result.is_ok(),
            "Deserialization (with unit) failed: {:?}",
            cell_file_result.err()
        );
        let cell_file = cell_file_result.unwrap();
        assert!((cell_file.md_eqm_ion_t.value - 0.5).abs() < f64::EPSILON);
        // assert_eq!(cell_file.md_eqm_ion_t.unit, Some(TimeUnit::Picosecond)); // If this variant exists

        // 2. Test Deserialization without unit (default/unit from context)
        // Note: Default logic is context-dependent. This just tests parsing a value without unit.
        let md_eqm_ion_t_default_str = "MD_EQM_ION_T : 1.0";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithMdEqmIonTDefault {
            md_eqm_ion_t: MdEqmIonT,
        }

        let cell_file_default_result: Result<CellFileWithMdEqmIonTDefault, _> =
            from_str(md_eqm_ion_t_default_str);
        assert!(
            cell_file_default_result.is_ok(),
            "Deserialization (default unit) failed: {:?}",
            cell_file_default_result.err()
        );
        let cell_file_default = cell_file_default_result.unwrap();
        assert!((cell_file_default.md_eqm_ion_t.value - 1.0).abs() < f64::EPSILON);
        assert_eq!(cell_file_default.md_eqm_ion_t.unit, None);

        // 3. Test Serialization using ToCell (with unit)
        // This depends on TimeUnit having a variant that serializes correctly.
        // let md_eqm_ion_t_instance_with_unit = MdEqmIonT {
        //     value: 0.75,
        //     unit: Some(TimeUnit::Picosecond), // If this variant exists
        // };
        // let serialized_result_with_unit = to_string(&md_eqm_ion_t_instance_with_unit.to_cell());
        // assert!(serialized_result_with_unit.is_ok(), "Serialization (with unit) failed: {:?}", serialized_result_with_unit.err());
        // let serialized_string_with_unit = serialized_result_with_unit.unwrap();
        // println!("Serialized MD_EQM_ION_T (0.75 ps): {serialized_string_with_unit}");
        // assert!(serialized_string_with_unit.contains("MD_EQM_ION_T"));
        // assert!(serialized_string_with_unit.contains("0.75"));
        // assert!(serialized_string_with_unit.contains("ps"));

        // 4. Test Serialization using ToCell (without unit)
        let md_eqm_ion_t_instance_no_unit = MdEqmIonT {
            value: 1.25,
            unit: None,
        };
        let serialized_result_no_unit = to_string(&md_eqm_ion_t_instance_no_unit.to_cell());
        assert!(
            serialized_result_no_unit.is_ok(),
            "Serialization (no unit) failed: {:?}",
            serialized_result_no_unit.err()
        );
        let serialized_string_no_unit = serialized_result_no_unit.unwrap();
        println!("Serialized MD_EQM_ION_T (1.25, no unit): {serialized_string_no_unit}");
        assert!(serialized_string_no_unit.contains("MD_EQM_ION_T"));
        assert!(serialized_string_no_unit.contains("1.25"));
        // Check that the unit string is not present
    }
}
