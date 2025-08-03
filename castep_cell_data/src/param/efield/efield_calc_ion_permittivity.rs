use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
use serde::{Deserialize, Serialize};

/// Controls whether to compute the zero-frequency dielectric permittivity based on ionic response.
///
/// Keyword type: Logical
///
/// Default: true
///
/// Example:
/// EFIELD_CALC_ION_PERMITTIVITY : FALSE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "EFIELD_CALC_ION_PERMITTIVITY")]
pub struct EfieldCalcIonPermittivity(pub bool);

impl Default for EfieldCalcIonPermittivity {
    fn default() -> Self {
        Self(true) // Default is TRUE
    }
}

impl ToCell for EfieldCalcIonPermittivity {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFIELD_CALC_ION_PERMITTIVITY", CellValue::Bool(self.0))
    }
}

impl ToCellValue for EfieldCalcIonPermittivity {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_efield_calc_ion_permittivity_serde() {
        // 1. Test Deserialization TRUE
        let efield_calc_ion_permittivity_true_str = "EFIELD_CALC_ION_PERMITTIVITY : TRUE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithEfieldCalcIonPermittivityTrue {
            efield_calc_ion_permittivity: EfieldCalcIonPermittivity,
        }

        let cell_file_true_result: Result<CellFileWithEfieldCalcIonPermittivityTrue, _> =
            from_str(efield_calc_ion_permittivity_true_str);
        assert!(
            cell_file_true_result.is_ok(),
            "Deserialization (TRUE) failed: {:?}",
            cell_file_true_result.err()
        );
        let cell_file_true = cell_file_true_result.unwrap();
        assert!(cell_file_true.efield_calc_ion_permittivity.0);

        // 2. Test Deserialization FALSE
        let efield_calc_ion_permittivity_false_str = "EFIELD_CALC_ION_PERMITTIVITY : FALSE";
        #[derive(Debug, Deserialize, Serialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct CellFileWithEfieldCalcIonPermittivityFalse {
            efield_calc_ion_permittivity: EfieldCalcIonPermittivity,
        }

        let cell_file_false_result: Result<CellFileWithEfieldCalcIonPermittivityFalse, _> =
            from_str(efield_calc_ion_permittivity_false_str);
        assert!(
            cell_file_false_result.is_ok(),
            "Deserialization (FALSE) failed: {:?}",
            cell_file_false_result.err()
        );
        let cell_file_false = cell_file_false_result.unwrap();
        assert!(!cell_file_false.efield_calc_ion_permittivity.0);

        // 3. Test Serialization using ToCell
        let efield_calc_ion_permittivity_instance = EfieldCalcIonPermittivity(false);
        let serialized_result = to_string(&efield_calc_ion_permittivity_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized EFIELD_CALC_ION_PERMITTIVITY (FALSE): {serialized_string}"); // Clippy suggestion
        assert!(serialized_string.contains("EFIELD_CALC_ION_PERMITTIVITY"));
        assert!(serialized_string.contains("false") || serialized_string.contains("FALSE"));

        // 4. Test Default
        assert_eq!(
            EfieldCalcIonPermittivity::default(),
            EfieldCalcIonPermittivity(true)
        );
    }
}
